use std::io::Write;
use std::fmt::Debug;
use std::fs::{metadata, read, read_dir, File};
use std::path::{Path, PathBuf};
use zip::CompressionMethod;
use zip::write::{FileOptions, ZipWriter};

fn add_mimetype(zip_file: &mut ZipWriter<File>) -> Result<(), String> {
    let mimetype_options = FileOptions::default().compression_method(CompressionMethod::Stored);
    zip_file.start_file("mimetype", mimetype_options).map_err(|e| e.to_string())?;
    zip_file.write(b"application/epub+zip").map_err(|e| e.to_string())?;

    Ok(())
}

fn add_subdir_member<P: AsRef<Path> + Clone + Debug>(zip_file: &mut ZipWriter<File>, path: P, mut path_within_zip_file: PathBuf) -> Result<(), String> {
    let path_metadata = metadata(path.clone()).map_err(|e| e.to_string())?;
    path_within_zip_file.push(path.as_ref().file_name().ok_or(format!("Ill-formed path ending in '..': {:?}", path))?);

    if path_metadata.is_file() {
        let file_contents = read(path.clone()).map_err(|e| e.to_string())?;
        zip_file.start_file(path_within_zip_file.into_os_string().into_string().map_err(|e| format!("{:?}", e))?, FileOptions::default()).map_err(|e| e.to_string())?;
        zip_file.write(&file_contents).map_err(|e| e.to_string())?;
    } else if path_metadata.is_dir() {
        for dir_entry in read_dir(path.clone()).map_err(|e| e.to_string())? {
            let dir_entry_path = dir_entry.map_err(|e| e.to_string())?.path();
            add_subdir_member(zip_file, dir_entry_path, path_within_zip_file.clone())?;
        }
    }

    Ok(())
}

fn add_root_level_file_or_dir<P: AsRef<Path> + Clone + Debug>(zip_file: &mut ZipWriter<File>, path: P) -> Result<(), String> {
    let path_metadata = metadata(path.clone()).map_err(|e| e.to_string())?;
    let file_or_dir_name = path.as_ref().file_name().ok_or(format!("Ill-formed path ending in '..': {:?}", path))?.to_str().ok_or(format!("Ill-formed path, bad unicode: {:?}", path))?;

    if path_metadata.is_file() {
        let file_contents = read(path.clone()).map_err(|e| e.to_string())?;
        zip_file.start_file(file_or_dir_name, FileOptions::default()).map_err(|e| e.to_string())?;
        zip_file.write(&file_contents).map_err(|e| e.to_string())?;
    } else if path_metadata.is_dir() {
        for dir_entry in read_dir(path.clone()).map_err(|e| e.to_string())? {
            let dir_entry_path = dir_entry.map_err(|e| e.to_string())?.path();
            add_subdir_member(zip_file, dir_entry_path, PathBuf::from(file_or_dir_name))?;
        }
    }

    Ok(())
}

pub fn zip_epub(output_filename: String, input_files_and_dirs: Vec<String>) -> Result<(), String> {
    let epub_file = File::create(output_filename).map_err(|e| e.to_string())?;
    let mut zip_file = ZipWriter::new(epub_file);

    add_mimetype(&mut zip_file)?;
    for path in input_files_and_dirs {
        add_root_level_file_or_dir(&mut zip_file, path)?
    }

    zip_file.finish().map_err(|e| e.to_string())?;

    Ok(())
}
