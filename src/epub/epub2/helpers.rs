use crate::epub::epub2::config::Epub2Config;

pub(crate)fn get_manifest_path_from_idref(config: &Epub2Config, idref: &str, fragment: Option<&String>) -> Result<String, String> {
    let manifest_item = match config.manifest.iter().find(|item| &item.id == idref) {
        Some(item) => item,
        None => return Err(format!("Idref {} not found in manifest.", idref)),
    };
    match fragment {
        None => Ok(manifest_item.inside_path_from_opf.clone()),
        Some(fragment) => Ok(format!("{}#{}", manifest_item.inside_path_from_opf, fragment)),
    }
}
