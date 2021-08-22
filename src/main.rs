use argh::FromArgs;
use std::io::Write;
use std::fs::{metadata, read, read_dir, File};
use std::path::{Path, PathBuf};
use zip::CompressionMethod;
use zip::write::{FileOptions, ZipWriter};

/// Build EPUB files. [ADD MORE DETAIL HERE]
#[derive(FromArgs)]
struct Args {
    /// output filename
    #[argh(positional)]
    out_filename: String,
    /// input files and directories to zip
    #[argh(positional)]
    in_paths: Vec<String>
}

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
    }
    else if path_metadata.is_dir() {
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
    }
    else if path_metadata.is_dir() {
        for dir_entry in read_dir(path.clone()).unwrap() {
            let dir_entry_path = dir_entry.unwrap().path();
            add_subdir_member(zip_file, dir_entry_path, PathBuf::from(file_or_dir_name))
        }
    }
}

fn main() {
    let args: Args = argh::from_env();

    let epub_file = File::create(args.out_filename).unwrap();
    let mut zip_file = ZipWriter::new(epub_file);

    add_mimetype(&mut zip_file);
    for path in args.in_paths {
        add_root_level_file_or_dir(&mut zip_file, &path)
    }

    zip_file.finish().unwrap();
}
