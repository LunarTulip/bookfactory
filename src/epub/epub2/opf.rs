use crate::epub::epub2::config;
use crate::epub::epub2::config::Epub2Config;
use crate::epub::epub2::helpers::get_path_from_idref;

use sys_locale::get_locale;
use uuid::Uuid;
use yaserde_derive::YaSerialize;

//////////////////
//   Metadata   //
//////////////////

#[derive(YaSerialize)]
struct Title {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Identifier {
    #[yaserde(attribute)]
    id: Option<String>,
    #[yaserde(attribute, rename = "opf:scheme")]
    opf_scheme: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Language {
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Creator {
    #[yaserde(attribute, rename: "opf:file-as")]
    opf_file_as: Option<String>,
    #[yaserde(attribute, rename: "opf:role")]
    opf_role: Option<String>,
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Subject {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Description {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Publisher {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Contributor {
    #[yaserde(attribute, rename: "opf:file-as")]
    opf_file_as: Option<String>,
    #[yaserde(attribute, rename: "opf:role")]
    opf_role: Option<String>,
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Date {
    #[yaserde(attribute, rename: "opf:event")]
    opf_event: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Type {
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Format {
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Source {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Relation {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Coverage {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Rights {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(text)]
    body: String,
}

#[derive(YaSerialize)]
struct Meta {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    content: String,
}

#[derive(YaSerialize)]
enum MetadataItem {
    // THIS IS CURRENTLY BROKEN DUE TO YASERDE ENUM ISSUES; IT DOESN'T OUTPUT WELL-FORMED METADATA AT THIS TIME. COME BACK AND FIX IT BEFORE ANY REAL RELEASE.
    #[yaserde(rename = "dc:title")]
    DcTitle(Title),
    #[yaserde(rename = "dc:identifier")]
    DcIdentifier(Identifier),
    #[yaserde(rename = "dc:language")]
    DcLanguage(Language),
    #[yaserde(rename = "dc:creator")]
    DcCreator(Creator),
    #[yaserde(rename = "dc:subject")]
    DcSubject(Subject),
    #[yaserde(rename = "dc:description")]
    DcDescription(Description),
    #[yaserde(rename = "dc:publisher")]
    DcPublisher(Publisher),
    #[yaserde(rename = "dc:contributor")]
    DcContributor(Contributor),
    #[yaserde(rename = "dc:date")]
    DcDate(Date),
    #[yaserde(rename = "dc:type")]
    DcType(Type),
    #[yaserde(rename = "dc:format")]
    DcFormat(Format),
    #[yaserde(rename = "dc:source")]
    DcSource(Source),
    #[yaserde(rename = "dc:relation")]
    DcRelation(Relation),
    #[yaserde(rename = "dc:coverage")]
    DcCoverage(Coverage),
    #[yaserde(rename = "dc:rights")]
    DcRights(Rights),
    #[yaserde(rename = "meta")]
    Meta(Meta),
}

#[derive(YaSerialize)]
struct Metadata {
    #[yaserde(attribute, rename = "xmlns:dc")]
    xmlns_dc: Option<String>,
    #[yaserde(attribute, rename = "xmlns:opf")]
    xmlns_opf: Option<String>,
    #[yaserde(child, flatten)]
    metadata: Vec<MetadataItem>,
}

//////////////////
//   Manifest   //
//////////////////

#[derive(YaSerialize)]
struct Item {
    #[yaserde(attribute)]
    id: String,
    #[yaserde(attribute)]
    href: String,
    #[yaserde(attribute, rename = "media-type")]
    media_type: String,
    #[yaserde(attribute)]
    fallback: Option<String>,
    #[yaserde(attribute, rename = "fallback-style")]
    fallback_style: Option<String>,
    #[yaserde(attribute, rename = "required-namespace")]
    required_namespace: Option<String>,
    #[yaserde(attribute, rename = "required-modules")]
    required_modules: Option<String>,
}

#[derive(YaSerialize)]
struct Manifest {
    #[yaserde(child)]
    item: Vec<Item>,
}

///////////////
//   Spine   //
///////////////

#[derive(YaSerialize)]
struct Itemref {
    #[yaserde(attribute)]
    linear: Option<String>,
    #[yaserde(attribute)]
    idref: String,
}

#[derive(YaSerialize)]
struct Spine {
    #[yaserde(attribute)]
    toc: String,
    #[yaserde(child)]
    itemref: Vec<Itemref>,
}

///////////////
//   Guide   //
///////////////

#[derive(YaSerialize)]
struct Reference {
    #[yaserde(attribute, rename = "type")]
    reference_type: String,
    #[yaserde(attribute)]
    title: Option<String>,
    #[yaserde(attribute)]
    href: String,
}

#[derive(YaSerialize)]
struct Guide {
    #[yaserde(child)]
    reference: Vec<Reference>,
}

/////////////////
//   Package   //
/////////////////

#[derive(YaSerialize)]
#[yaserde(rename = "package")]
struct Package {
    #[yaserde(attribute)]
    version: String,
    #[yaserde(attribute)]
    xmlns: String,
    #[yaserde(attribute, rename = "unique-identifier")]
    unique_identifier: String,
    #[yaserde(child)]
    metadata: Metadata,
    #[yaserde(child)]
    manifest: Manifest,
    #[yaserde(child)]
    spine: Spine,
    #[yaserde(child)]
    guide: Option<Guide>,
}

///////////////
//   Build   //
///////////////

fn get_uid_and_title_and_metadata(
    config: &Epub2Config,
    safe_uid: &str,
) -> Result<(String, String, Metadata), String> {
    match &config.metadata {
        Some(config_metadata) => {
            let mut metadata = Vec::new();

            // Sort all metadata from the config

            for item in config_metadata {
                match item {
                    config::Metadata::DcMetadata {name, content, id, scheme, file_as, role, event, lang} => {
                        match name.as_ref() {
                            "title" =>  metadata.push(MetadataItem::DcTitle(Title {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "identifier" => metadata.push(MetadataItem::DcIdentifier(Identifier {
                                id: id.clone(),
                                opf_scheme: scheme.clone(),
                                body: content.clone(),
                            })),
                            "language" => metadata.push(MetadataItem::DcLanguage(Language {
                                body: content.clone(),
                            })),
                            "creator" => metadata.push(MetadataItem::DcCreator(Creator {
                                opf_file_as: file_as.clone(),
                                opf_role: role.clone(),
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "subject" => metadata.push(MetadataItem::DcSubject(Subject {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "description" => metadata.push(MetadataItem::DcDescription(Description {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "publisher" => metadata.push(MetadataItem::DcPublisher(Publisher {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "contributor" => metadata.push(MetadataItem::DcContributor(Contributor {
                                opf_file_as: file_as.clone(),
                                opf_role: role.clone(),
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "date" => metadata.push(MetadataItem::DcDate(Date {
                                opf_event: event.clone(),
                                body: content.clone(),
                            })),
                            "type" => metadata.push(MetadataItem::DcType(Type {
                                body: content.clone(),
                            })),
                            "format" => metadata.push(MetadataItem::DcFormat(Format {
                                body: content.clone(),
                            })),
                            "source" => metadata.push(MetadataItem::DcSource(Source {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "relation" => metadata.push(MetadataItem::DcRelation(Relation {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "coverage" => metadata.push(MetadataItem::DcCoverage(Coverage {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            "rights" => metadata.push(MetadataItem::DcRights(Rights {
                                xml_lang: lang.clone(),
                                body: content.clone(),
                            })),
                            _ => return Err(format!("Unrecognized DC metadata name: '{}'; if using custom metadata names, please use the attribute custom_name in place of name.", name)),
                        }
                    }
                    config::Metadata::CustomMetadata {name, content} => metadata.push(MetadataItem::Meta(Meta {
                        name: name.clone(),
                        content: content.clone(),
                    })),
                }
            }

            // Generate any missing required metadata

            if !metadata.iter().any(|item| {
                if let MetadataItem::DcTitle(_) = item {
                    true
                } else {
                    false
                }
            }) {
                metadata.push(MetadataItem::DcTitle(Title {
                    xml_lang: None,
                    body: String::from("Untitled"),
                }))
            };

            if !metadata.iter().any(|item| {
                if let MetadataItem::DcIdentifier(_) = item {
                    true
                } else {
                    false
                }
            }) {
                metadata.push(MetadataItem::DcIdentifier(Identifier {
                    id: Some(String::from(safe_uid)),
                    opf_scheme: Some(String::from("UUID")),
                    body: format!("{}", Uuid::new_v4()),
                }))
            }

            if !metadata.iter().any(|item| {
                if let MetadataItem::DcLanguage(_) = item {
                    true
                } else {
                    false
                }
            }) {
                metadata.push(MetadataItem::DcLanguage(Language {
                    body: match get_locale() {
                        Some(locale) => locale,
                        None => String::from("en"),
                    },
                }))
            };

            // Get UID and title and return

            let uid = match metadata.iter().find(|item| {
                if let MetadataItem::DcIdentifier(identifier) = item {
                    identifier.id.is_some()
                } else {
                    false
                }
            }) {
                Some(MetadataItem::DcIdentifier(identifier)) => identifier.id.clone().unwrap(),
                _ => {
                    match metadata.iter_mut().find(|item| if let MetadataItem::DcIdentifier(_) = item { true } else { false }) {
                        Some(MetadataItem::DcIdentifier(identifier)) => {
                            identifier.id = Some(String::from(safe_uid));
                            String::from(safe_uid)
                        },
                        _ => return Err(String::from("No identifier found after identifier's presence should have been ensured.")),
                    }
                }
            };

            let title = match metadata.iter().find(|item| {
                if let MetadataItem::DcTitle(_) = item {
                    true
                } else {
                    false
                }
            }) {
                Some(MetadataItem::DcTitle(title)) => title.body.clone(),
                _ => {
                    return Err(String::from(
                        "No title found after title's presence should have been ensured.",
                    ))
                }
            };

            let metadata = Metadata {
                xmlns_dc: Some(String::from("http://purl.org/dc/elements/1.1/")),
                xmlns_opf: Some(String::from("http://www.idpf.org/2007/opf")),
                metadata: metadata,
            };

            Ok((uid, title, metadata))
        }
        None => {
            let metadata = Metadata {
                xmlns_dc: Some(String::from("http://purl.org/dc/elements/1.1/")),
                xmlns_opf: Some(String::from("http://www.idpf.org/2007/opf")),
                metadata: vec![
                    MetadataItem::DcTitle(Title {
                        xml_lang: None,
                        body: String::from("Untitled"),
                    }),
                    MetadataItem::DcIdentifier(Identifier {
                        id: Some(String::from(safe_uid)),
                        opf_scheme: Some(String::from("UUID")),
                        body: format!("{}", Uuid::new_v4()),
                    }),
                    MetadataItem::DcLanguage(Language {
                        body: match get_locale() {
                            Some(locale) => locale,
                            None => String::from("en"),
                        },
                    }),
                ],
            };

            Ok((String::from(safe_uid), String::from("Untitled"), metadata))
        }
    }
}

fn get_manifest(config: &Epub2Config, ncx_id: &str, ncx_path_from_opf: &str) -> Manifest {
    let mut items_vec = vec![Item {
        id: String::from(ncx_id),
        href: String::from(ncx_path_from_opf),
        media_type: String::from("application/x-dtbncx+xml"),
        fallback: None,
        fallback_style: None,
        required_modules: None,
        required_namespace: None,
    }];

    items_vec.append(
        &mut config
            .manifest
            .iter()
            .map(|item| Item {
                id: item.id.clone(),
                href: item.inside_path_from_opf.clone(),
                media_type: item.media_type.clone(),
                fallback: item.fallback.clone(),
                fallback_style: item.fallback_style.clone(),
                required_modules: item.required_modules.clone(),
                required_namespace: item.required_namespace.clone(),
            })
            .collect(),
    );

    Manifest { item: items_vec }
}

fn id_falls_back_to_types(config: &Epub2Config, id: &str, target_types: &Vec<&str>) -> bool {
    match config.manifest.iter().find(|item| &item.id == id) {
        Some(item) => {
            if target_types.contains(&item.media_type.as_ref()) {
                true
            } else if item.fallback.is_some() {
                id_falls_back_to_types(config, &item.fallback.as_ref().unwrap(), target_types)
            } else {
                false
            }
        }
        None => false,
    }
}

fn get_spine(config: &Epub2Config, ncx_id: &str) -> Result<Spine, String> {
    let first_linearizable_manifest_item = match config.manifest.iter().find(|item| {
        &item.media_type == "application/xhtml+xml"
            || &item.media_type == "application/x-dtbook+xml"
            || id_falls_back_to_types(
                config,
                &item.id,
                &vec!["application/xhtml+xml", "application/x-dtbook+xml"],
            )
    }) {
        Some(item) => item,
        None => {
            return Err(String::from(
                "Manifest contains no items legally placeable within the spine.",
            ))
        }
    };

    let mut itemrefs = Vec::new();
    match &config.spine {
        Some(spine) => {
            for itemref in spine {
                let (idref, linear) = match itemref {
                    config::Itemref::RawIdref(idref) => (idref, None),
                    config::Itemref::CookedIdref { idref, linear } => (
                        idref,
                        match linear {
                            Some(false) => Some(String::from("no")),
                            _ => None,
                        },
                    ),
                };
                itemrefs.push(Itemref {
                    linear: linear,
                    idref: idref.clone(),
                });
            }
        }
        None => itemrefs.push(Itemref {
            idref: first_linearizable_manifest_item.id.clone(),
            linear: None,
        }),
    }

    Ok(Spine {
        toc: String::from(ncx_id),
        itemref: itemrefs,
    })
}

fn get_guide(config: &Epub2Config) -> Result<Option<Guide>, String> {
    match &config.guide {
        None => Ok(None),
        Some(guide) => {
            let mut references = Vec::new();
            for reference in guide {
                references.push(Reference {
                    reference_type: reference.reference_type.clone(),
                    title: reference.title.clone(),
                    href: get_path_from_idref(
                        config,
                        &reference.idref,
                        reference.fragment.as_ref(),
                    )?,
                });
            }
            Ok(Some(Guide {
                reference: references,
            }))
        }
    }
}

pub(crate) fn build_opf_xml_and_get_metadata(
    config: &Epub2Config,
    ncx_id: &str,
    ncx_path_from_opf: &str,
    safe_uid: &str,
) -> Result<(String, String, String, String), String> {
    let (uid, title, metadata) = get_uid_and_title_and_metadata(&config, safe_uid)?;
    let opf = Package {
        version: String::from("2.0"),
        xmlns: String::from("http://www.idpf.org/2007/opf"),
        unique_identifier: uid.clone(),
        metadata: metadata,
        manifest: get_manifest(&config, ncx_id, ncx_path_from_opf),
        spine: get_spine(&config, ncx_id)?,
        guide: get_guide(&config)?,
    };

    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };
    let opf_xml = yaserde::ser::to_string_with_config(&opf, &yaserde_cfg)?;

    let first_linear_spine_href = match opf
        .spine
        .itemref
        .iter()
        .find(|itemref| itemref.linear.is_some())
    {
        None => {
            return Err(String::from(
                "No linear items found in spine after their presence should be guaranteed.",
            ))
        }
        Some(itemref) => get_path_from_idref(config, &itemref.idref, None)?,
    };

    Ok((opf_xml, uid, title, first_linear_spine_href))
}
