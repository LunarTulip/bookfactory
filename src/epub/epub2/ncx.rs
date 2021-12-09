use crate::epub::epub2::config::Epub2Config;

use yaserde_derive::YaSerialize;

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
struct NavMap {

}

//////////////////
//   PageList   //
//////////////////


#[derive(YaSerialize)]
struct PageList {

}

/////////////////
//   NavList   //
/////////////////

#[derive(YaSerialize)]
struct NavList {

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
    navlist: Option<Vec<NavList>>,
}

///////////////
//   Build   //
///////////////

pub(crate) fn build_ncx_xml(config: &Epub2Config) -> Result<String, String> {
    // TODO

    Ok(String::new())
}
