use build_epub::zip::zip_epub;

use argh::FromArgs;

/// Zip all EPUB source files minus the mimetype into a well-formed EPUB.
#[derive(FromArgs)]
struct Args {
    /// output filename
    #[argh(positional)]
    out_filename: String,
    /// input files and directories to zip
    #[argh(positional)]
    in_paths: Vec<String>,
}

fn main() {
    let args: Args = argh::from_env();

    zip_epub(args.out_filename, args.in_paths).unwrap();
}
