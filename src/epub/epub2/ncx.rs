use crate::epub::epub2::config;
use crate::epub::epub2::config::Epub2Config;
use crate::epub::epub2::helpers::get_path_from_idref;
use crate::helpers::fixed_clean;

use common_path::common_path;
use std::path::PathBuf;
use yaserde_derive::YaSerialize;

///////////////////
//   Multi-Use   //
///////////////////

#[derive(YaSerialize)]
struct NavLabel {
    #[yaserde(attribute, rename = "xml:lang")]
    xml_lang: Option<String>,
    #[yaserde(child)]
    text: String,
}

//////////////
//   Head   //
//////////////

#[derive(YaSerialize)]
struct Meta {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    content: String,
}

#[derive(YaSerialize)]
struct Head {
    #[yaserde(child)]
    meta: Meta,
}

//////////////////
//   DocTitle   //
//////////////////

#[derive(YaSerialize)]
struct DocTitle {
    #[yaserde(child)]
    text: String,
}

////////////////
//   NavMap   //
////////////////

#[derive(YaSerialize)]
struct NavPoint {
    #[yaserde(child, rename = "navLabel")]
    navlabel: Vec<NavLabel>,
    #[yaserde(child)]
    content: String,
    #[yaserde(child, rename = "navPoint")]
    navpoint: Vec<NavPoint>, // Should be an option, but a library bug is interfering
}

#[derive(YaSerialize)]
struct NavMap {
    #[yaserde(child, rename = "navPoint")]
    navpoint: Vec<NavPoint>,
}

//////////////////
//   PageList   //
//////////////////

#[derive(YaSerialize)]
struct PageTarget {
    #[yaserde(attribute)]
    id: String,
    #[yaserde(attribute, rename = "type")]
    pagetarget_type: String,
    #[yaserde(attribute)]
    value: Option<String>,
    #[yaserde(child, rename = "navLabel")]
    navlabel: Vec<NavLabel>,
    #[yaserde(child)]
    content: String,
}

#[derive(YaSerialize)]
struct PageList {
    #[yaserde(child, rename = "pageTarget")]
    pagetarget: Vec<PageTarget>,
}

/////////////////
//   NavList   //
/////////////////

#[derive(YaSerialize)]
struct NavTarget {
    #[yaserde(child, rename = "navLabel")]
    navlabel: Vec<NavLabel>,
    #[yaserde(child)]
    content: String,
}

#[derive(YaSerialize)]
struct NavList {
    #[yaserde(child, rename = "navLabel")]
    navlabel: Vec<NavLabel>,
    #[yaserde(child, rename = "navTarget")]
    navtarget: Vec<NavTarget>,
}

/////////////
//   NCX   //
/////////////

#[derive(YaSerialize)]
#[yaserde(rename = "ncx")]
struct Ncx {
    #[yaserde(attribute)]
    version: String,
    #[yaserde(attribute)]
    xmlns: String,
    #[yaserde(child)]
    head: Head,
    #[yaserde(child, rename = "docTitle")]
    doctitle: DocTitle,
    #[yaserde(child, rename = "navMap")]
    navmap: NavMap,
    #[yaserde(child, rename = "pageList")]
    pagelist: Option<PageList>,
    #[yaserde(child, rename = "navList")]
    navlist: Vec<NavList>, // Should be an option, but a library bug is interfering
}

/////////////////
//   Helpers   //
/////////////////

fn get_ncx_path_to_file(
    opf_parent_path_from_zip_root: &PathBuf,
    ncx_path_from_opf: &PathBuf,
    file_path_from_opf: &PathBuf,
) -> Result<String, String> {
    let ncx_path_from_zip_root = {
        let mut path = opf_parent_path_from_zip_root.clone();
        path.push(&ncx_path_from_opf);
        fixed_clean(path)
    };
    let file_path_from_zip_root = {
        let mut path = opf_parent_path_from_zip_root.clone();
        path.push(&file_path_from_opf);
        fixed_clean(path)
    };
    let filename = {
        let filename_os_str = file_path_from_opf.file_name().ok_or(format!(
            "Invalid path ending in '..': {}",
            file_path_from_opf.display()
        ))?;
        filename_os_str
            .to_str()
            .ok_or(format!("Invalid non-UTF-8 filename: {:?}", filename_os_str))?
    };

    if ncx_path_from_zip_root.parent() == file_path_from_zip_root.parent() {
        // File is in same folder as NCX
        Ok(format!("{}", filename))
    } else {
        match file_path_from_zip_root.parent() {
            None => {
                Err(format!("File {} has no parent. (This shouldn't be able to happen, since 'the empty parent' is counted separately from 'no parent'.)", file_path_from_zip_root.display()))
            }
            Some(file_parent) => {
                if ncx_path_from_zip_root.starts_with(file_parent) {
                    // File is in a non-root ancestor of the NCX's parent folder
                    let ncx_depth_from_file_parent = ncx_path_from_zip_root.strip_prefix(file_parent).map_err(|e| e.to_string())?.ancestors().count() - 2; // Subtract 1 for the method counting the NCX as its own ancestor and 1 for the empty ancestor

                    let mut prefix = String::new();
                    for _ in 0..ncx_depth_from_file_parent {
                        prefix = format!("../{}", prefix);
                    }

                    Ok(format!("{}{}", prefix, filename))
                } else {
                    match common_path(&ncx_path_from_zip_root, &file_path_from_zip_root) {
                        Some(common_ancestor) => {
                            // File is in a different branch of the file tree from the NCX, diverging below the zip root
                            let ncx_depth_from_common_ancestor = ncx_path_from_zip_root.strip_prefix(&common_ancestor).map_err(|e| e.to_string())?.ancestors().count() - 2;
                            let file_path_from_common_ancestor = {
                                let path_os_str = file_path_from_zip_root.strip_prefix(&common_ancestor).map_err(|e| e.to_string())?.as_os_str();
                                path_os_str.to_str().ok_or(format!("Invalid non-UTF-8 path: {:?}", path_os_str))?
                            };

                            let mut prefix = String::new();
                            for _ in 0..ncx_depth_from_common_ancestor {
                                prefix = format!("../{}", prefix);
                            }

                            Ok(format!("{}{}", prefix, file_path_from_common_ancestor))
                        }
                        None => {
                            // File is in the zip root, or else in a different branch of the file tree from the NCX which diverges at the zip root
                            let ncx_depth = ncx_path_from_zip_root.ancestors().count() - 2;

                            let mut prefix = String::new();
                            for _ in 0..ncx_depth {
                                prefix = format!("../{}", prefix);
                            }

                            Ok(format!("{}{}", prefix, file_path_from_zip_root.as_os_str().to_str().ok_or(format!("Invalid non-UTF-8 path: {:?}", file_path_from_zip_root))?))
                        }
                    }
                }
            }
        }
    }
}

fn get_ncx_path_to_file_from_idref(
    config: &Epub2Config,
    idref: &str,
    fragment: Option<&String>,
    opf_parent_abs_path: &PathBuf,
    ncx_path_from_opf: &PathBuf,
) -> Result<String, String> {
    let file_path_from_opf = get_path_from_idref(config, idref, fragment)?;
    get_ncx_path_to_file(
        opf_parent_abs_path,
        ncx_path_from_opf,
        &PathBuf::from(file_path_from_opf),
    )
}

///////////////
//   Build   //
///////////////

fn convert_navpoint(
    config: &Epub2Config,
    opf_parent_path: &PathBuf,
    ncx_path_from_opf: &PathBuf,
    other_format_navpoint: &config::NavPoint,
) -> Result<NavPoint, String> {
    Ok(match other_format_navpoint {
        config::NavPoint::WithSimpleLabel {
            label,
            idref,
            fragment,
            children,
        } => NavPoint {
            navlabel: vec![NavLabel {
                xml_lang: None,
                text: label.clone(),
            }],
            content: get_ncx_path_to_file_from_idref(
                config,
                &idref,
                fragment.as_ref(),
                opf_parent_path,
                ncx_path_from_opf,
            )?,
            navpoint: match children {
                None => Vec::new(),
                Some(children) => {
                    let mut children_vec = Vec::new();
                    for child in children {
                        children_vec.push(convert_navpoint(
                            config,
                            opf_parent_path,
                            ncx_path_from_opf,
                            child,
                        )?);
                    }
                    children_vec
                }
            },
        },
        config::NavPoint::WithComplexLabels {
            labels,
            idref,
            fragment,
            children,
        } => NavPoint {
            navlabel: labels
                .iter()
                .map(|label| NavLabel {
                    xml_lang: label.lang.clone(),
                    text: label.label.clone(),
                })
                .collect(),
            content: get_ncx_path_to_file_from_idref(
                config,
                &idref,
                fragment.as_ref(),
                opf_parent_path,
                ncx_path_from_opf,
            )?,
            navpoint: match children {
                None => Vec::new(),
                Some(children) => {
                    let mut children_vec = Vec::new();
                    for child in children {
                        children_vec.push(convert_navpoint(
                            config,
                            opf_parent_path,
                            ncx_path_from_opf,
                            child,
                        )?);
                    }
                    children_vec
                }
            },
        },
    })
}

fn get_navmap(
    config: &Epub2Config,
    opf_parent_path: &PathBuf,
    ncx_path_from_opf: &PathBuf,
    doctitle: &str,
    first_linear_spine_href: &str,
) -> Result<NavMap, String> {
    Ok(match &config.navmap {
        None => NavMap {
            navpoint: vec![NavPoint {
                navlabel: vec![NavLabel {
                    xml_lang: None,
                    text: String::from(doctitle),
                }],
                content: String::from(first_linear_spine_href),
                navpoint: Vec::new(),
            }],
        },
        Some(navmap) => NavMap {
            navpoint: {
                let mut navpoints_vec = Vec::new();
                for navpoint in navmap {
                    navpoints_vec.push(convert_navpoint(
                        config,
                        opf_parent_path,
                        ncx_path_from_opf,
                        navpoint,
                    )?);
                }
                navpoints_vec
            },
        },
    })
}

fn get_pagelist(
    config: &Epub2Config,
    opf_parent_path: &PathBuf,
    ncx_path_from_opf: &PathBuf,
) -> Result<Option<PageList>, String> {
    match &config.pagelist {
        None => Ok(None),
        Some(pagelist) => Ok(Some(PageList {
            pagetarget: {
                let mut pagetargets_vec = Vec::new();
                for pagetarget in pagelist {
                    pagetargets_vec.push(match pagetarget {
                        config::PageTarget::WithSimpleLabel {
                            label,
                            id,
                            target_type,
                            value,
                            idref,
                            fragment,
                        } => PageTarget {
                            id: id.clone(),
                            pagetarget_type: target_type.clone(),
                            value: value.clone(),
                            navlabel: vec![NavLabel {
                                xml_lang: None,
                                text: label.clone(),
                            }],
                            content: get_ncx_path_to_file_from_idref(
                                config,
                                &idref,
                                fragment.as_ref(),
                                opf_parent_path,
                                ncx_path_from_opf,
                            )?,
                        },
                        config::PageTarget::WithComplexLabels {
                            labels,
                            id,
                            target_type,
                            value,
                            idref,
                            fragment,
                        } => PageTarget {
                            id: id.clone(),
                            pagetarget_type: target_type.clone(),
                            value: value.clone(),
                            navlabel: labels
                                .iter()
                                .map(|label| NavLabel {
                                    xml_lang: label.lang.clone(),
                                    text: label.label.clone(),
                                })
                                .collect(),
                            content: get_ncx_path_to_file_from_idref(
                                config,
                                &idref,
                                fragment.as_ref(),
                                opf_parent_path,
                                ncx_path_from_opf,
                            )?,
                        },
                    });
                }
                pagetargets_vec
            },
        })),
    }
}

fn convert_navlist(
    config: &Epub2Config,
    opf_parent_path: &PathBuf,
    ncx_path_from_opf: &PathBuf,
    other_format_list: &Vec<config::NavTarget>,
) -> Result<Vec<NavTarget>, String> {
    let mut navtarget_vec = Vec::new();
    for navtarget in other_format_list {
        navtarget_vec.push(match navtarget {
            config::NavTarget::WithSimpleLabel {
                label,
                idref,
                fragment,
            } => NavTarget {
                navlabel: vec![NavLabel {
                    xml_lang: None,
                    text: label.clone(),
                }],
                content: get_ncx_path_to_file_from_idref(
                    config,
                    &idref,
                    fragment.as_ref(),
                    opf_parent_path,
                    ncx_path_from_opf,
                )?,
            },
            config::NavTarget::WithComplexLabels {
                labels,
                idref,
                fragment,
            } => NavTarget {
                navlabel: labels
                    .iter()
                    .map(|label| NavLabel {
                        xml_lang: label.lang.clone(),
                        text: label.label.clone(),
                    })
                    .collect(),
                content: get_ncx_path_to_file_from_idref(
                    config,
                    &idref,
                    fragment.as_ref(),
                    opf_parent_path,
                    ncx_path_from_opf,
                )?,
            },
        });
    }
    Ok(navtarget_vec)
}

fn get_navlists(
    config: &Epub2Config,
    opf_parent_path: &PathBuf,
    ncx_path_from_opf: &PathBuf,
) -> Result<Vec<NavList>, String> {
    match &config.navlists {
        None => Ok(Vec::new()),
        Some(navlists) => {
            let mut navlists_vec = Vec::new();
            for navlist in navlists {
                navlists_vec.push(match navlist {
                    config::NavList::WithSimpleLabel { label, list } => NavList {
                        navlabel: vec![NavLabel {
                            xml_lang: None,
                            text: label.clone(),
                        }],
                        navtarget: convert_navlist(
                            config,
                            opf_parent_path,
                            ncx_path_from_opf,
                            list,
                        )?,
                    },
                    config::NavList::WithComplexLabels { labels, list } => NavList {
                        navlabel: labels
                            .iter()
                            .map(|label| NavLabel {
                                xml_lang: label.lang.clone(),
                                text: label.label.clone(),
                            })
                            .collect(),
                        navtarget: convert_navlist(
                            config,
                            opf_parent_path,
                            ncx_path_from_opf,
                            list,
                        )?,
                    },
                });
            }
            Ok(navlists_vec)
        }
    }
}

pub(crate) fn build_ncx_xml(
    config: &Epub2Config,
    opf_parent_path: &PathBuf,
    ncx_path_from_opf: &PathBuf,
    uid: &str,
    doctitle: &str,
    first_linear_spine_href: &str,
) -> Result<String, String> {
    let clean_opf_parent_path = &fixed_clean(opf_parent_path);

    let ncx = Ncx {
        version: String::from("2005-1"),
        xmlns: String::from("http://www.daisy.org/z3986/2005/ncx/"),
        head: Head {
            meta: Meta {
                name: String::from("dtb:uid"),
                content: String::from(uid),
            },
        },
        doctitle: DocTitle {
            text: String::from(doctitle),
        },
        navmap: get_navmap(
            config,
            clean_opf_parent_path,
            ncx_path_from_opf,
            doctitle,
            first_linear_spine_href,
        )?,
        pagelist: get_pagelist(config, clean_opf_parent_path, ncx_path_from_opf)?,
        navlist: get_navlists(config, clean_opf_parent_path, ncx_path_from_opf)?,
    };

    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };

    yaserde::ser::to_string_with_config(&ncx, &yaserde_cfg)
}
