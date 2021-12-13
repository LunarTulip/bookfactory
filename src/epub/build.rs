use crate::epub::zip::add_epub_mimetype;
use crate::zip::zip_path;

use std::io::Cursor;
use zip::write::ZipWriter;

pub fn zip_with_epub_mimetype(in_paths: Vec<String>) -> Result<Vec<u8>, String> {
    let mut file_buffer = Vec::<u8>::new();
    let mut zip_file = ZipWriter::new(Cursor::new(&mut file_buffer));

    add_epub_mimetype(&mut zip_file)?;
    for path in in_paths {
        zip_path(&mut zip_file, path, None::<String>)?;
    }

    zip_file.finish().map_err(|e| e.to_string())?;
    drop(zip_file);

    Ok(file_buffer)
}