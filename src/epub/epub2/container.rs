use crate::epub::epub2::config::Epub2Config;

use yaserde_derive::YaSerialize;

#[derive(YaSerialize)]
struct Rootfile {
    #[yaserde(attribute, rename = "full-path")]
    full_path: String,
    #[yaserde(attribute, rename = "media-type")]
    media_type: String,
}

#[derive(YaSerialize)]
struct Rootfiles {
    #[yaserde(child)]
    rootfile: Vec<Rootfile>,
}

#[derive(YaSerialize)]
#[yaserde(rename = "container")]
struct Container {
    #[yaserde(attribute)]
    version: String,
    #[yaserde(attribute)]
    xmlns: String,
    #[yaserde(child)]
    rootfiles: Rootfiles,
}

pub(crate) fn build_container_xml(config: &Epub2Config, add_opf_to_rootfiles: bool) -> Result<String, String> {
    let container = Container {
        version: String::from("1.0"),
        xmlns: String::from("urn:oasis:names:tc:opendocument:xmlns:container"),
        rootfiles: Rootfiles {
            rootfile: match &config.rootfiles {
                None => vec![Rootfile {
                    full_path: String::from("OEBPS/content.opf"),
                    media_type: String::from("application/oebps-package+xml"),
                }],
                Some(rootfiles_vec) => {
                    let mut xml_rootfiles_vec: Vec<Rootfile> = rootfiles_vec.iter().map(|rootfile| Rootfile {
                        full_path: rootfile.path.clone(),
                        media_type: rootfile.media_type.clone(),
                    }).collect();
                    if add_opf_to_rootfiles {
                        xml_rootfiles_vec.push(Rootfile {
                            full_path: String::from("OEBPS/content.opf"),
                            media_type: String::from("application/oebps-package+xml"),
                        });
                    }

                    xml_rootfiles_vec
                }
            }
        }
    };

    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };

    yaserde::ser::to_string_with_config(&container, &yaserde_cfg)
}
