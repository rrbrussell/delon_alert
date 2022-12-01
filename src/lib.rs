use yaserde_derive::{YaDeserialize, YaSerialize};

pub mod repomd;
pub use repomd::{Repomd, RepomdXmlDataEntry};

pub mod primary;

#[derive(Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Checksum {
    #[yaserde(attribute, rename = "type")]
    algorithm: String,
    #[yaserde(attribute)]
    pkgid: Option<bool>,
    #[yaserde(text)]
    value: String,
}

#[derive(Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Location {
    #[yaserde(attribute)]
    href: String,
}
