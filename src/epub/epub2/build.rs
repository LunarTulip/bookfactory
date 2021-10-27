use crate::epub::epub2::container::build_container_xml;
use crate::epub::zip::add_epub_mimetype;
use crate::toml::parse_config::parse_config;
use crate::zip::{zip_buffer, zip_path};
use std::io::Cursor;
use std::mem::drop;
use zip::write::ZipWriter;

pub fn build_epub2(config_path: String) -> Result<Vec<u8>, String> {
    // Set up zip file
    let config = parse_config(config_path)?;

    let mut epub_file_buffer = Vec::<u8>::new();
    let mut zip_file = ZipWriter::new(Cursor::new(&mut epub_file_buffer));

    // Load up zip file with appropriate contents
    add_epub_mimetype(&mut zip_file)?;
    for path_pair in &config.files_to_store {
        zip_path(&mut zip_file, &path_pair.outside_path, Some(&path_pair.inside_path))?;
    }

    let container_xml = build_container_xml(config)?;
    zip_buffer(&mut zip_file, container_xml.as_bytes().to_vec(), "META-INF/container.xml")?;

    // The rest of the epub goes here once I define its methods

    // Wrap up file and return
    zip_file.finish().map_err(|e| e.to_string())?;
    drop(zip_file);

    Ok(epub_file_buffer)
}
