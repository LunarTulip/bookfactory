use bookfactory::epub::{build_epub2, zip_with_epub_mimetype};
use bookfactory::toml::{parse_config, Recipe};

use argh::FromArgs;
use std::fs::write;

//////////////
//   Args   //
//////////////

/// Build ebook according to the input config file
#[derive(FromArgs)]
#[argh(subcommand, name = "build")]
struct Build {
    /// output path
    #[argh(positional)]
    out_path: String,
    /// input config file
    #[argh(positional)]
    config_file: String,
    /// recipe to build from config file
    #[argh(positional)]
    recipe_name: String,
}

/// Zip input paths with epub mimetype
#[derive(FromArgs)]
#[argh(subcommand, name = "zip_epub")]
struct ZipEpub {
    /// output path
    #[argh(positional)]
    out_path: String,
    /// input paths
    #[argh(positional)]
    in_paths: Vec<String>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Build(Build),
    ZipEpub(ZipEpub),
}

/// BookFactory ebook-building tool
#[derive(FromArgs)]
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

//////////////
//   Main   //
//////////////

enum Format {
    Epub2,
    Unrecognized,
}

fn get_format(recipe: &Recipe) -> Format {
    match recipe.format.as_ref() {
        "epub2" => Format::Epub2,
        _ => Format::Unrecognized,
    }
}

fn build(args: Build) -> Result<(), String> {
    let recipes = parse_config(&args.config_file)?;

    let file = match recipes
        .iter()
        .find(|recipe| recipe.name == args.recipe_name)
    {
        None => {
            return Err(format!(
                "Recipe {} not found in file {}.",
                args.recipe_name, args.config_file
            ))
        }
        Some(recipe) => match get_format(recipe) {
            Format::Epub2 => build_epub2(recipe).unwrap(),
            Format::Unrecognized => {
                return Err(format!(
                    "Format {} not recognized in recipe {}",
                    recipe.format, recipe.name
                ))
            }
        },
    };
    write(args.out_path, file).map_err(|e| e.to_string())?;

    Ok(())
}

fn zip_epub(args: ZipEpub) -> Result<(), String> {
    let file = zip_with_epub_mimetype(args.in_paths)?;
    write(args.out_path, file).map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    let args: Args = argh::from_env();
    let result = match args.subcommand {
        Subcommand::Build(command) => build(command),
        Subcommand::ZipEpub(command) => zip_epub(command),
    };
    match result {
        Ok(_) => println!("Book built successfully."),
        Err(e) => println!("Error encountered during build:\n{}", e),
    }
}
