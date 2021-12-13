use std::io::{Seek, Write};
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

pub(crate) fn add_epub_mimetype<Z: Write + Seek>(zip_file: &mut ZipWriter<Z>) -> Result<(), String> {
    let mimetype_options = FileOptions::default().compression_method(CompressionMethod::Stored);
    zip_file
        .start_file("mimetype", mimetype_options)
        .map_err(|e| e.to_string())?;
    zip_file
        .write(b"application/epub+zip")
        .map_err(|e| e.to_string())?;

    Ok(())
}
