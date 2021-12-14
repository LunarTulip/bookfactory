use crate::epub::epub2::config::{parse_epub2_recipe, Metadata, PageTarget};
use crate::epub::epub2::{
    container::build_container_xml, ncx::build_ncx_xml, opf::build_opf_xml_and_get_metadata,
};
use crate::epub::zip::add_epub_mimetype;
use crate::toml::Recipe;
use crate::zip::{zip_buffer, zip_path};

use std::io::Cursor;
use std::mem::drop;
use std::path::{Path, PathBuf};
use zip::write::ZipWriter;

fn check_no_id_collisions(ids: &Vec<String>) -> Result<(), String> {
    let mut ids_as_str: Vec<&str> = ids.iter().map(|id| id.as_ref()).collect();
    ids_as_str.sort();
    ids_as_str.dedup_by(|a, b| a.eq_ignore_ascii_case(b));

    match ids_as_str.len() == ids.len() {
        true => Ok(()),
        false => Err(String::from(
            "Attempted to use multiple copies of the same ID in the same file.",
        )),
    }
}

fn get_safe_uid(opf_ids: &Vec<String>) -> String {
    let mut tentative_id = String::from("BookId");
    let mut number_to_append = 1;
    while opf_ids.contains(&tentative_id) {
        tentative_id = format!("{}_{}", "BookId", number_to_append);
        number_to_append += 1;
    }
    tentative_id
}

fn check_no_duplicate_inside_paths(inside_paths: &Vec<PathBuf>) -> Result<(), String> {
    let mut paths_as_str = inside_paths
        .iter()
        .map(|path| match path.to_str() {
            None => Err(format!("Invalid non-UTF-8 path: {:?}", path)),
            Some(in_str) => Ok(in_str),
        })
        .collect::<Result<Vec<&str>, String>>()?;
    paths_as_str.sort();
    paths_as_str.dedup_by(|a, b| a.eq_ignore_ascii_case(b));

    match paths_as_str.len() == inside_paths.len() {
        true => Ok(()),
        false => Err(String::from("Attempted to store multiple files at the same inside path, or at inside paths differing only by case.")),
    }
}

fn check_inside_path_is_valid(inside_path: &PathBuf) -> Result<(), String> {
    match inside_path.file_name() {
        None => return Err(format!("Invalid path ending in '..': {:?}", inside_path)),
        Some(filename) => match filename.to_str() {
            None => return Err(format!("Invalid non-UTF-8 filename: {:?}", filename)),
            Some(filename_str) => {
                if filename_str.len() > 255 {
                    return Err(format!(
                        "Invalid filename of length >255 bytes: {}",
                        filename_str
                    ));
                }
            }
        },
    };

    match inside_path.to_str() {
        None => return Err(format!("Invalid non-UTF-8 path: {:?}", inside_path)),
        Some(path_str) => {
            if path_str.len() > 65535 {
                return Err(format!("Invalid path of length >65535 bytes: {}", path_str));
            }
        }
    };

    for component in inside_path.iter() {
        match component.to_str() {
            None => {
                return Err(format!(
                    "Invalid non-UTF-8 path component (you shouldn't ever see this error): {:?}",
                    component
                ))
            }
            Some(component_str) => {
                match component_str.find(&['/', '"', '*', ':', '<', '>', '?', '\\'][..]) {
                    None => (),
                    Some(index) => {
                        return Err(format!(
                            "Path {:?} contains invalid character '{}'.",
                            inside_path,
                            component_str.get(index..index + 1).unwrap()
                        ))
                    }
                };
                if component_str.ends_with('.') {
                    return Err(format!("Path {:?} ends with '.'.", inside_path));
                }
            }
        };
    }

    Ok(())
}

pub fn build_epub2(recipe: &Recipe) -> Result<Vec<u8>, String> {
    // Parse recipe into build config and various derivatives thereof
    let config = parse_epub2_recipe(recipe)?;
    let (add_opf_to_rootfiles, opf_path) = match &config.rootfiles {
        None => (true, "OEBPS/content.opf"),
        Some(rootfiles_vec) => match rootfiles_vec
            .iter()
            .find(|rootfile| &rootfile.media_type == "application/oebps-package+xml")
        {
            None => (true, "OEBPS/content.opf"),
            Some(rootfile) => (false, rootfile.path.as_ref()),
        },
    };
    let opf_parent_dir = match Path::new(opf_path).parent() {
        None => Path::new(""),
        Some(parent) => parent,
    };
    let (ncx_id, ncx_path_from_opf) = match &config.ncx_meta {
        Some(meta) => {
            let id = match &meta.manifest_id {
                Some(id) => id,
                None => "ncx",
            };
            let path = match &meta.manifest_path_from_opf {
                Some(path) => path,
                None => "toc.ncx",
            };
            (id, path)
        }
        None => ("ncx", "toc.ncx"),
    };
    let mut ncx_path = PathBuf::from(opf_parent_dir);
    ncx_path.push(ncx_path_from_opf);

    // Set up zip file
    let mut epub_file_buffer = Vec::<u8>::new();
    let mut zip_file = ZipWriter::new(Cursor::new(&mut epub_file_buffer));

    // Validate paths
    let mut outside_and_inside_paths = Vec::new();

    for item in &config.manifest {
        let mut item_inside_path = PathBuf::new();
        item_inside_path.push(opf_parent_dir);
        item_inside_path.push(&item.inside_path_from_opf);
        outside_and_inside_paths.push((item.outside_path.as_ref(), item_inside_path));
    }

    if let Some(nonmanifest_file_vec) = &config.nonmanifest_files {
        for nonmanifest_file in nonmanifest_file_vec {
            outside_and_inside_paths.push((
                nonmanifest_file.outside_path.as_ref(),
                PathBuf::from(&nonmanifest_file.inside_path),
            ));
        }
    }

    let mut inside_paths: Vec<PathBuf> = outside_and_inside_paths
        .iter()
        .map(|(_outside, inside): &(&str, PathBuf)| inside.clone())
        .collect();
    inside_paths.append(&mut vec![
        PathBuf::from("META-INF/container.xml"),
        PathBuf::from(&opf_path),
        PathBuf::from(&ncx_path),
    ]);

    check_no_duplicate_inside_paths(&inside_paths)?;
    for inside_path in inside_paths {
        check_inside_path_is_valid(&inside_path)?;
    }

    // Validate IDs
    let mut opf_ids: Vec<String> = config.manifest.iter().map(|item| item.id.clone()).collect();
    opf_ids.push(String::from(ncx_id));
    if let Some(metadata) = &config.metadata {
        opf_ids.append(
            &mut metadata
                .iter()
                .filter_map(|item| match item {
                    Metadata::DcMetadata { id, .. } => id.clone(),
                    Metadata::CustomMetadata { .. } => None,
                })
                .collect(),
        );
    }

    check_no_id_collisions(&opf_ids)?;
    let safe_uid = get_safe_uid(&opf_ids);

    if let Some(list) = &config.pagelist {
        let ncx_ids = list
            .iter()
            .map(|target| match target {
                PageTarget::WithSimpleLabel { id, .. } => id.clone(),
                PageTarget::WithComplexLabels { id, .. } => id.clone(),
            })
            .collect();
        check_no_id_collisions(&ncx_ids)?;
    }

    // Generate non-preexisting files
    let container_xml = build_container_xml(&config, add_opf_to_rootfiles)?;
    let (opf_xml, uid, title, first_linear_spine_href) =
        build_opf_xml_and_get_metadata(&config, &ncx_id, &ncx_path_from_opf, &safe_uid)?;
    let ncx_xml = build_ncx_xml(&config, &uid, &title, &first_linear_spine_href)?;

    // Zip up all files
    add_epub_mimetype(&mut zip_file)?;
    for (outside_path, inside_path) in outside_and_inside_paths {
        zip_path(&mut zip_file, outside_path, Some(inside_path))?;
    }
    zip_buffer(
        &mut zip_file,
        container_xml.as_bytes().to_vec(),
        "META-INF/container.xml",
    )?;
    zip_buffer(&mut zip_file, opf_xml.as_bytes().to_vec(), opf_path)?;
    zip_buffer(&mut zip_file, ncx_xml.as_bytes().to_vec(), ncx_path)?;

    // Wrap up and return
    zip_file.finish().map_err(|e| e.to_string())?;
    drop(zip_file);

    Ok(epub_file_buffer)
}
