use bookfactory::epub::build_epub2;
use bookfactory::toml::*;

use argh::FromArgs;
use std::fs::write;

/// Build ebook according to the input config file.
#[derive(FromArgs)]
struct Args {
    /// output filename
    #[argh(positional)]
    out_filename: String,
    /// input config file
    #[argh(positional)]
    config_file: String,
    /// recipe to build from config file
    #[argh(positional)]
    recipe_name: String,
}

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

fn main() {
    let args: Args = argh::from_env();
    let recipes = parse_config(&args.config_file).unwrap();

    let file = match recipes
        .iter()
        .find(|recipe| recipe.name == args.recipe_name)
    {
        None => panic!(
            "Recipe {} not found in file {}.",
            args.recipe_name, args.config_file
        ),
        Some(recipe) => match get_format(recipe) {
            Format::Epub2 => build_epub2(recipe).unwrap(),
            Format::Unrecognized => panic!(
                "Format {} not recognized in recipe {}",
                recipe.format, recipe.name
            ),
        },
    };
    write(args.out_filename, file).unwrap();
}
