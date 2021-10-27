use build_epub::epub::build_epub2;

use argh::FromArgs;
use std::fs::write;

/// Build epub according to the input config file.
#[derive(FromArgs)]
struct Args {
    /// output filename
    #[argh(positional)]
    out_filename: String,
    /// input config file
    #[argh(positional)]
    config_file: String,
}

fn main() {
    let args: Args = argh::from_env();
    let file = build_epub2(args.config_file).unwrap();
    write(args.out_filename, file).unwrap();
}
