use crate::epub::epub2::config;
use crate::epub::epub2::config::Epub2Config;
use crate::epub::epub2::helpers::get_manifest_path_from_idref;

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

///////////////
//   Build   //
///////////////

fn convert_navpoint(
    config: &Epub2Config,
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
            content: get_manifest_path_from_idref(config, &idref, fragment.as_ref())?,
            navpoint: match children {
                None => Vec::new(),
                Some(children) => {
                    let mut children_vec = Vec::new();
                    for child in children {
                        children_vec.push(convert_navpoint(config, child)?);
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
            content: get_manifest_path_from_idref(config, &idref, fragment.as_ref())?,
            navpoint: match children {
                None => Vec::new(),
                Some(children) => {
                    let mut children_vec = Vec::new();
                    for child in children {
                        children_vec.push(convert_navpoint(config, child)?);
                    }
                    children_vec
                }
            },
        },
    })
}

fn get_navmap(
    config: &Epub2Config,
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
                    navpoints_vec.push(convert_navpoint(config, navpoint)?);
                }
                navpoints_vec
            },
        },
    })
}

fn get_pagelist(config: &Epub2Config) -> Result<Option<PageList>, String> {
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
                            content: get_manifest_path_from_idref(
                                config,
                                &idref,
                                fragment.as_ref(),
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
                            content: get_manifest_path_from_idref(
                                config,
                                &idref,
                                fragment.as_ref(),
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
                content: get_manifest_path_from_idref(config, &idref, fragment.as_ref())?,
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
                content: get_manifest_path_from_idref(config, &idref, fragment.as_ref())?,
            },
        });
    }
    Ok(navtarget_vec)
}

fn get_navlists(config: &Epub2Config) -> Result<Vec<NavList>, String> {
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
                        navtarget: convert_navlist(config, list)?,
                    },
                    config::NavList::WithComplexLabels { labels, list } => NavList {
                        navlabel: labels
                            .iter()
                            .map(|label| NavLabel {
                                xml_lang: label.lang.clone(),
                                text: label.label.clone(),
                            })
                            .collect(),
                        navtarget: convert_navlist(config, list)?,
                    },
                });
            }
            Ok(navlists_vec)
        }
    }
}

pub(crate) fn build_ncx_xml(
    config: &Epub2Config,
    uid: &str,
    doctitle: &str,
    first_linear_spine_href: &str,
) -> Result<String, String> {
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
        navmap: get_navmap(config, doctitle, first_linear_spine_href)?,
        pagelist: get_pagelist(config)?,
        navlist: get_navlists(config)?,
    };

    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };

    yaserde::ser::to_string_with_config(&ncx, &yaserde_cfg)
}
