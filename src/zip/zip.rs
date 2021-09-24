use std::fmt::Debug;
use std::fs::{metadata, read, read_dir, File};
use std::io::{Cursor, Write};
use std::mem::drop;
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

fn p_to_string<P: AsRef<Path> + Clone + Debug>(p: P) -> Result<String, String> {
    Ok(String::from(
        p.as_ref()
            .as_os_str()
            .to_str()
            .ok_or(format!("Ill-formed path, bad unicode: {:?}", p))?,
    ))
}

fn add_file_with_optional_deflate<P: AsRef<Path> + Clone + Debug>(
    zip_file: &mut ZipWriter<File>,
    file_contents: Vec<u8>,
    inside_path: P,
) -> Result<(), String> {
    let mut compression_test_buffer = Vec::new();
    let mut compression_test_writer = ZipWriter::new(Cursor::new(&mut compression_test_buffer));

    let uncompressed_size = file_contents.len() as u64;

    compression_test_writer
        .start_file(p_to_string(&inside_path)?, FileOptions::default())
        .map_err(|e| e.to_string())?;
    compression_test_writer
        .write(&file_contents)
        .map_err(|e| e.to_string())?;
    compression_test_writer
        .finish()
        .map_err(|e| e.to_string())?;
    drop(compression_test_writer);

    let mut compression_test_reader =
        ZipArchive::new(Cursor::new(&mut compression_test_buffer)).map_err(|e| e.to_string())?;
    let file_in_zip = compression_test_reader
        .by_name(&p_to_string(&inside_path)?)
        .map_err(|e| e.to_string())?;
    let compressed_size = file_in_zip.compressed_size();

    if compressed_size < uncompressed_size {
        zip_file
            .raw_copy_file(file_in_zip)
            .map_err(|e| e.to_string())?;
    } else {
        zip_file
            .start_file(
                p_to_string(inside_path)?,
                FileOptions::default().compression_method(CompressionMethod::Stored),
            )
            .map_err(|e| e.to_string())?;
        zip_file.write(&file_contents).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn zip_path<P: AsRef<Path> + Clone + Debug, 
Q: AsRef<Path> + Clone + Debug>(
    zip_file: &mut ZipWriter<File>,
    outside_path: P,
    inside_path: Option<Q>
) -> Result<(), String> {
    let path_metadata = metadata(outside_path.clone()).map_err(|e| e.to_string())?;
    let true_inside_path = match inside_path {
        Some(path) => {
            let mut as_buf = path.as_ref().to_path_buf();
            as_buf.push(outside_path.as_ref().file_name().ok_or(format!(
                "Ill-formed path ending in '..': {:?}",
                outside_path
            ))?);
            as_buf
        }
        None => PathBuf::from(outside_path.as_ref().file_name().ok_or(format!("Ill-formed path ending in '..': {:?}", outside_path))?)
    };

    if path_metadata.is_file() {
        let file_contents = read(outside_path).map_err(|e| e.to_string())?;
        add_file_with_optional_deflate(zip_file, file_contents, true_inside_path)?;
    } else if path_metadata.is_dir() {
        for dir_entry in read_dir(outside_path).map_err(|e| e.to_string())? {
            let dir_entry_path = dir_entry.map_err(|e| e.to_string())?.path();
            zip_path(zip_file, dir_entry_path, Some(true_inside_path.clone()))?;
        }
    }

    Ok(())
}

pub fn zip_buffer<P: AsRef<Path> + Clone + Debug>(zip_file: &mut ZipWriter<File>, buffer: Vec<u8>, inside_path: P) -> Result<(), String> {
    add_file_with_optional_deflate(zip_file, buffer, inside_path)
}
