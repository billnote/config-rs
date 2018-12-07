// If no features are used, there is an "unused mut" warning in `ALL_EXTENSIONS`
// BUG: ? For some reason this doesn't do anything if I try and function scope this
#![allow(unused_mut)]

use source::Source;
use std::collections::HashMap;
use std::error::Error;
use value::Value;
use case;

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "hjson")]
mod hjson;

#[cfg(feature = "ini")]
mod ini;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FileFormat {
    /// TOML (parsed with toml)
    #[cfg(feature = "toml")]
    Toml,

    /// JSON (parsed with serde_json)
    #[cfg(feature = "json")]
    Json,

    /// YAML (parsed with yaml_rust)
    #[cfg(feature = "yaml")]
    Yaml,

    /// HJSON (parsed with serde_hjson)
    #[cfg(feature = "hjson")]
    Hjson,
    /// INI (parsed with rust_ini)
    #[cfg(feature = "ini")]
    Ini,
}

lazy_static! {
    #[doc(hidden)]
    // #[allow(unused_mut)] ?
    pub static ref ALL_EXTENSIONS: HashMap<FileFormat, Vec<&'static str>> = {
        let mut formats: HashMap<FileFormat, Vec<_>> = HashMap::new();

        #[cfg(feature = "toml")]
        formats.insert(FileFormat::Toml, vec!["toml"]);

        #[cfg(feature = "json")]
        formats.insert(FileFormat::Json, vec!["json"]);

        #[cfg(feature = "yaml")]
        formats.insert(FileFormat::Yaml, vec!["yaml", "yml"]);

        #[cfg(feature = "hjson")]
        formats.insert(FileFormat::Hjson, vec!["hjson"]);

        #[cfg(feature = "ini")]
        formats.insert(FileFormat::Ini, vec!["ini"]);

        formats
    };
}

pub fn insert_all_case_key(map: &mut HashMap<String, Value>, key: &str, value: &Value) {
    map.insert(case::RenameRule::CamelCase.apply_to_field(key).to_owned(), value.clone());
    map.insert(case::RenameRule::KebabCase.apply_to_field(key).to_owned(), value.clone());
    map.insert(case::RenameRule::LowerCase.apply_to_field(key).to_owned(), value.clone());
    map.insert(case::RenameRule::PascalCase.apply_to_field(key).to_owned(), value.clone());
    map.insert(case::RenameRule::ScreamingKebabCase.apply_to_field(key).to_owned(), value.clone());
    map.insert(case::RenameRule::ScreamingSnakeCase.apply_to_field(key).to_owned(), value.clone());
    map.insert(case::RenameRule::SnakeCase.apply_to_field(key).to_owned(), value.clone());
    map.insert(case::RenameRule::UPPERCASE.apply_to_field(key).to_owned(), value.clone());
    map.insert(key.to_owned(), value.clone());
}

impl FileFormat {
    // TODO: pub(crate)
    #[doc(hidden)]
    pub fn extensions(&self) -> &'static Vec<&'static str> {
        // It should not be possible for this to fail
        // A FileFormat would need to be declared without being added to the
        // ALL_EXTENSIONS map.
        ALL_EXTENSIONS.get(self).unwrap()
    }

    // TODO: pub(crate)
    #[doc(hidden)]
    #[allow(unused_variables)]
    pub fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<HashMap<String, Value>, Box<Error + Send + Sync>> {
        match *self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => toml::parse(uri, text),

            #[cfg(feature = "json")]
            FileFormat::Json => json::parse(uri, text),

            #[cfg(feature = "yaml")]
            FileFormat::Yaml => yaml::parse(uri, text),

            #[cfg(feature = "hjson")]
            FileFormat::Hjson => hjson::parse(uri, text),

            #[cfg(feature = "ini")]
            FileFormat::Ini => ini::parse(uri, text),
        }
    }
}
