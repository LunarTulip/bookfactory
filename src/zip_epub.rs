use std::io::Write;
use std::fs::{metadata, read, read_dir, File};
use std::path::{Path, PathBuf};
use zip::CompressionMethod;
use zip::write::{FileOptions, ZipWriter};

fn add_mimetype(zip_file: &mut ZipWriter<File>) {
    let mimetype_options = FileOptions::default().compression_method(CompressionMethod::Stored);
    zip_file.start_file("mimetype", mimetype_options).unwrap();
    zip_file.write(b"application/epub+zip").unwrap();
}

fn add_subdir_member<P: AsRef<Path> + Clone>(zip_file: &mut ZipWriter<File>, path: P, mut path_within_zip_file: PathBuf) {
    let path_metadata = metadata(path.clone()).unwrap();
    path_within_zip_file.push(path.as_ref().file_name().unwrap());

    if path_metadata.is_file() {
        let file_contents = read(path.clone()).unwrap();
        zip_file.start_file(path_within_zip_file.into_os_string().into_string().unwrap(), FileOptions::default()).unwrap();
        zip_file.write(&file_contents).unwrap();
    } else if path_metadata.is_dir() {
        for dir_entry in read_dir(path.clone()).unwrap() {
            let dir_entry_path = dir_entry.unwrap().path();
            add_subdir_member(zip_file, dir_entry_path, path_within_zip_file.clone())
        }
    }
}

fn add_root_level_file_or_dir<P: AsRef<Path> + Clone>(zip_file: &mut ZipWriter<File>, path: P) {
    let path_metadata = metadata(path.clone()).unwrap();
    let file_or_dir_name = path.as_ref().file_name().unwrap().to_str().unwrap();

    if path_metadata.is_file() {
        let file_contents = read(path.clone()).unwrap();
        zip_file.start_file(file_or_dir_name, FileOptions::default()).unwrap();
        zip_file.write(&file_contents).unwrap();
    } else if path_metadata.is_dir() {
        for dir_entry in read_dir(path.clone()).unwrap() {
            let dir_entry_path = dir_entry.unwrap().path();
            add_subdir_member(zip_file, dir_entry_path, PathBuf::from(file_or_dir_name))
        }
    }
}

pub fn zip_epub(output_filename: String, input_files_and_dirs: Vec<String>) {
    let epub_file = File::create(output_filename).unwrap();
    let mut zip_file = ZipWriter::new(epub_file);

    add_mimetype(&mut zip_file);
    for path in input_files_and_dirs {
        add_root_level_file_or_dir(&mut zip_file, path)
    }

    zip_file.finish().unwrap();
}
