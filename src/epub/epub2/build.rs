use crate::epub::epub2::container::build_container_xml;
use crate::toml::parse_config::parse_config;

pub fn build_epub2(config_path: String) -> Result<(), String> {
    let config = parse_config(config_path)?;
    let container_xml = build_container_xml(config)?;

    println!("{}", container_xml);

    // In the long run, also take an output filename and save here

    Ok(())
}
