use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;
use toml::Value;

pub struct Recipe {
    pub name: String,
    pub format: String,
    pub recipe: Value,
}

pub fn parse_config<P: AsRef<Path> + Display>(filename: P) -> Result<Vec<Recipe>, String> {
    let file = read_to_string(&filename).map_err(|s| s.to_string())?;
    let config_tree = toml::from_str(&file).map_err(|s| s.to_string())?;

    if let Value::Table(top_level_table) = config_tree {
        let mut recipes = Vec::new();
        for (name, table) in top_level_table.iter() {
            if let Value::Table(recipe_table) = table {
                match recipe_table.get("format") {
                    Some(Value::String(format)) => {
                        let mut table_minus_format = recipe_table.clone();
                        table_minus_format.remove("format");
                        recipes.push(Recipe {
                            name: name.clone(),
                            format: format.clone(),
                            recipe: Value::Table(table_minus_format),
                        });
                    }
                    _ => return Err(format!("Recipe {} in config file {} contains no 'format' string value.", name, filename)),
                }
            } else {
                return Err(format!("Config file {} contains top-level element which is not a recipe header: {}", name, filename))
            }
        }
        if !recipes.is_empty() {
            Ok(recipes)
        } else {
            Err(format!("Config file {} contains no recipes.", filename))
        }
    } else {
        Err(format!("Config file {} has non-table top-level element. (This shouldn't be possible.)", filename))
    }
}
