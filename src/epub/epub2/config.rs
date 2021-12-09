use crate::toml::Recipe;

use serde::Deserialize;

///////////////////
//   Container   //
///////////////////

#[derive(Deserialize)]
pub(crate) struct Rootfile {
    pub(crate) path: String,
    #[serde(rename = "media-type")]
    pub(crate) media_type: String,
}

/////////////
//   OPF   //
/////////////

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum Metadata {
    DcMetadata {
        // Core
        name: String,
        content: String,

        // Attributes
        id: Option<String>, // For use with dc:identifier
        scheme: Option<String>, // opf:scheme
        #[serde(rename = "file-as")]
        file_as: Option<String>, // opf:file-as
        role: Option<String>, // opf:role
        event: Option<String>, // opf:event
        lang: Option<String>, // xml:lang
    },
    CustomMetadata {
        #[serde(rename = "custom_name")]
        name: String,
        content: String,
    },
}

#[derive(Deserialize)]
pub(crate) struct ManifestItem {
    // Core
    pub(crate) outside_path: String,
    pub(crate) inside_path_from_opf: String,
    #[serde(rename = "media-type")]
    pub(crate) media_type: String,
    pub(crate) id: String,

    // Fallback
    pub(crate) fallback: Option<String>,
    #[serde(rename = "fallback-style")]
    pub(crate) fallback_style: Option<String>,
    #[serde(rename = "required-namespace")]
    pub(crate) required_namespace: Option<String>,
    #[serde(rename = "required-modules")]
    pub(crate) required_modules: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum Itemref {
    RawIdref(String),
    CookedIdref {
        idref: String,
        linear: Option<bool>,
    },
}

#[derive(Deserialize)]
pub(crate) struct Reference {
    #[serde(rename = "type")]
    pub(crate) reference_type: String,
    pub(crate) title: Option<String>,
    pub(crate) idref: String,
    pub(crate) fragment: Option<String>,
}

/////////////
//   NCX   //
/////////////

#[derive(Deserialize)]
pub(crate) struct NcxMeta {
    pub(crate) manifest_id: Option<String>,
    pub(crate) manifest_path_from_opf: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct NavLabel {
    pub(crate) label: String,
    pub(crate) lang: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum NavPoint {
    WithSimpleLabel {
        label: String,
        idref: String,
        fragment: Option<String>,
        children: Option<Vec<NavPoint>>,
    },
    WithComplexLabels {
        labels: Vec<NavLabel>,
        idref: String,
        fragment: Option<String>,
        children: Option<Vec<NavPoint>>
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum PageTarget {
    WithSimpleLabel {
        label: String,
        id: String,
        #[serde(rename = "type")]
        target_type: String,
        value: Option<String>,
        idref: String,
        fragment: Option<String>,
    },
    WithComplexLabels {
        labels: Vec<NavLabel>,
        id: String,
        #[serde(rename = "type")]
        target_type: String,
        value: Option<String>,
        idref: String,
        fragment: Option<String>,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum NavTarget {
    WithSimpleLabel {
        label: String,
        idref: String,
        fragment: Option<String>,
    },
    WithComplexLabels {
        labels: Vec<NavLabel>,
        idref: String,
        fragment: Option<String>,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum NavList {
    WithSimpleLabel {
        label: String,
        list: Vec<NavTarget>,
    },
    WithComplexLabels {
        labels: Vec<NavLabel>,
        list: Vec<NavTarget>,
    }
}

///////////////////////
//   Miscellaneous   //
///////////////////////

#[derive(Deserialize)]
pub(crate) struct NonmanifestFile {
    pub(crate) outside_path: String,
    pub(crate) inside_path: String,
}

////////////////////////////
//   Main Config Struct   //
////////////////////////////

#[derive(Deserialize)]
pub(crate) struct Epub2Config {
    // Container
    pub(crate) rootfiles: Option<Vec<Rootfile>>,

    // OPF
    pub(crate) metadata: Option<Vec<Metadata>>,
    pub(crate) manifest: Vec<ManifestItem>,
    pub(crate) spine: Option<Vec<Itemref>>,
    pub(crate) guide: Option<Vec<Reference>>,

    // NCX
    pub(crate) ncx_meta: Option<NcxMeta>,
    pub(crate) navmap: Option<Vec<NavPoint>>,
    pub(crate) pagelist: Option<Vec<PageTarget>>,
    pub(crate) navlists: Option<Vec<NavList>>,

    // Miscellaneous
    pub(crate) nonmanifest_files: Option<Vec<NonmanifestFile>>,
}

pub(crate) fn parse_epub2_recipe(recipe: &Recipe) -> Result<Epub2Config, String> {
    let config = recipe.recipe.clone().try_into().map_err(|s| s.to_string())?;

    Ok(config)
}
