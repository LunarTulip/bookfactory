use crate::epub::epub2::config::parse_epub2_recipe;
use crate::epub::epub2::{
    container::build_container_xml, ncx::build_ncx_xml, opf::build_opf_xml_and_get_metadata,
};
use crate::epub::zip::add_epub_mimetype;
use crate::toml::Recipe;
use crate::zip::*;

use std::io::Cursor;
use std::mem::drop;
use std::path::{Path, PathBuf};
use zip::write::ZipWriter;

fn check_no_duplicate_inside_paths(paths_vec: &Vec<(&str, PathBuf)>) -> Result<(), String> {
    let mut inside_paths = paths_vec
        .iter()
        .map(|(_outside, inside)| match inside.to_str() {
            None => Err(format!("Invalid non-UTF-8 path: {:?}", inside)),
            Some(in_str) => Ok(in_str),
        })
        .collect::<Result<Vec<&str>, String>>()?;
    inside_paths.sort();
    inside_paths.dedup_by(|a, b| a.eq_ignore_ascii_case(b));

    match inside_paths.len() == paths_vec.len() {
        true => Ok(()),
        false => Err(String::from("Attempted to store multiple files at the same inside path, or at inside paths differing only by case.")),
    }
}

fn check_inside_path_is_valid(path: &PathBuf) -> Result<(), String> {
    match path.file_name() {
        None => return Err(format!("Invalid path ending in '..': {:?}", path)),
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

    match path.to_str() {
        None => return Err(format!("Invalid non-UTF-8 path: {:?}", path)),
        Some(path_str) => {
            if path_str.len() > 65535 {
                return Err(format!("Invalid path of length >65535 bytes: {}", path_str));
            }
        }
    };

    for component in path.iter() {
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
                            path,
                            component_str.get(index..index + 1).unwrap()
                        ))
                    }
                };
                if component_str.ends_with('.') {
                    return Err(format!("Path {:?} ends with '.'.", path));
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
        // In the long run, put more integrity-checking here, ensuring no collision with other manifest IDs and paths. (And do the same for IDs within the manifest with one another.)
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

    add_epub_mimetype(&mut zip_file)?;

    // Add preexisting files to zip file
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

    check_no_duplicate_inside_paths(&outside_and_inside_paths)?;
    for (outside_path, inside_path) in outside_and_inside_paths {
        check_inside_path_is_valid(&inside_path)?;
        zip_path(&mut zip_file, outside_path, Some(inside_path))?;
    }

    // Generate non-preexisting files and add them to zip file
    let container_xml = build_container_xml(&config, add_opf_to_rootfiles)?;
    zip_buffer(
        &mut zip_file,
        container_xml.as_bytes().to_vec(),
        "META-INF/container.xml",
    )?;

    let (opf_xml, uid, title, first_linear_spine_href) =
        build_opf_xml_and_get_metadata(&config, &ncx_id, &ncx_path_from_opf)?;
    zip_buffer(&mut zip_file, opf_xml.as_bytes().to_vec(), opf_path)?;

    let ncx_xml = build_ncx_xml(&config, &uid, &title, &first_linear_spine_href)?;
    zip_buffer(&mut zip_file, ncx_xml.as_bytes().to_vec(), ncx_path)?;

    // The rest of the epub goes here once I define its methods

    // Wrap up file and return
    zip_file.finish().map_err(|e| e.to_string())?;
    drop(zip_file);

    Ok(epub_file_buffer)
}
