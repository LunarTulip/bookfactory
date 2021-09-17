use crate::zip::zip_recursive;
use std::fs::File;
use std::io::Write;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

fn add_epub_mimetype(zip_file: &mut ZipWriter<File>) -> Result<(), String> {
    let mimetype_options = FileOptions::default().compression_method(CompressionMethod::Stored);
    zip_file
        .start_file("mimetype", mimetype_options)
        .map_err(|e| e.to_string())?;
    zip_file
        .write(b"application/epub+zip")
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn zip_epub(output_filename: String, input_files_and_dirs: Vec<String>) -> Result<(), String> {
    let epub_file = File::create(output_filename).map_err(|e| e.to_string())?;
    let mut zip_file = ZipWriter::new(epub_file);

    add_epub_mimetype(&mut zip_file)?;
    for path in input_files_and_dirs {
        zip_recursive(&mut zip_file, path)?;
    }

    zip_file.finish().map_err(|e| e.to_string())?;

    Ok(())
}
