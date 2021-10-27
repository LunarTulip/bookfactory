use serde::Deserialize;
use std::fs::read_to_string;

//////////////////////
//   Root Section   //
//////////////////////

#[derive(Deserialize)]
pub(crate) struct Rootfile {
    pub(crate) path: String,
    #[serde(rename = "media-type")]
    pub(crate) media_type: String,
}

#[derive(Deserialize)]
pub(crate) struct PathPair {
    pub(crate) outside_path: String,
    pub(crate) inside_path: String,
}

/////////////////////
//   OPF Section   //
/////////////////////

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

// Manifest entry struct goes here

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum Itemref {
    RawIdref(String),
    CookedIdref {
        idref: String,
        linear: Option<bool>,
    },
    Filename {
        filename: String,
        linear: Option<bool>,
    },
}

#[derive(Deserialize)]
pub(crate) struct Reference {
    #[serde(rename = "type")]
    pub(crate) reference_type: String,
    pub(crate) title: Option<String>,
    pub(crate) href: String, // Possibly change this into an idref/filename plus optional fragment identifier, to make it easier to abstract out internal paths
}

/////////////////////
//   NCX Section   //
/////////////////////

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
        content: String, // Possibly change this to idref/filename, same as the spine
        children: Option<Vec<NavPoint>>
    },
    WithComplexLabels {
        labels: Vec<NavLabel>,
        content: String,
        children: Option<Vec<NavPoint>>
    },
}

// PageTarget struct for NCX pageList goes here

// NavTarget struct for NCX navList goes here

////////////////////////////
//   Main Config Struct   //
////////////////////////////

#[derive(Deserialize)]
pub(crate) struct Config {
    // Root Section
    pub(crate) rootfiles: Option<Vec<Rootfile>>,
    pub(crate) files_to_store: Vec<PathPair>, // Should I try to make this optional?
    pub(crate) opf_internal_path: Option<String>,
    pub(crate) ncx_internal_path: Option<String>,

    // OPF Section
    pub(crate) metadata: Option<Vec<Metadata>>,
    // TODO: figure out manifest
    pub(crate) toc_id: Option<String>, // ID for the NCX linked from the spine
    pub(crate) spine: Vec<Itemref>, // Should I try to make this optional?
    pub(crate) guide: Option<Vec<Reference>>,

    // NCX Section
    pub(crate) navmap: Option<Vec<NavPoint>>
}

pub(crate) fn parse_config(filename: String) -> Result<Config, String> {
    let file = read_to_string(filename).map_err(|s| s.to_string())?;
    let config = toml::from_str(&file).map_err(|s| s.to_string())?;

    Ok(config)
}
