use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "repomd", namespace = "http://linux.duke.edu/metadata/repo")]
pub struct Repomd {
    #[yaserde(child)]
    revision: u64,
    #[yaserde(child)]
    data: Vec<RepomdXmlDataEntry>,
}

#[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "data", namespace = "http://linux.duke.edu/metadata/repo")]
pub struct RepomdXmlDataEntry {
    #[yaserde(attribute, rename = "type")]
    kind: String,
    #[yaserde(child)]
    checksum: Checksum,
    #[yaserde(child, rename = "open-checksum")]
    open_checksum: Option<Checksum>,
    #[yaserde(child, rename = "header-checksum")]
    header_checksum: Option<Checksum>,
    #[yaserde(child)]
    location: Location,
    #[yaserde(child)]
    timestamp: u64,
    #[yaserde(child)]
    size: u64,
    #[yaserde(rename = "open-size", child)]
    open_size: Option<u64>,
    #[yaserde(child)]
    database_version: Option<u64>,
    #[yaserde(rename = "header-size", child)]
    header_size: Option<u64>,
}

#[derive(Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Checksum {
    #[yaserde(attribute, rename = "type")]
    algorithm: String,
    #[yaserde(text)]
    value: String,
}

#[derive(Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
pub struct Location {
    #[yaserde(attribute)]
    href: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_reading() {
        let master_checksum = Checksum {
            algorithm: String::from("sha256"),
            value: String::from("947fa8d654d341e418467a40d33f7beb4474c612ca803cb36141d29b0d0101c1"),
        };
        let input_string = "<checksum type=\"sha256\">947fa8d654d341e418467a40d33f7beb4474c612ca803cb36141d29b0d0101c1</checksum>";
        let loaded_checksum: Checksum = yaserde::de::from_str(input_string).unwrap();

        assert_eq!(master_checksum, loaded_checksum);
    }

    #[test]
    fn test_repomd_xmldata_entry() {
        let master_entry = RepomdXmlDataEntry {
            kind: String::from("primary"),
            checksum: Checksum { algorithm: String::from("sha256"), value: String::from("54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3") },
            open_checksum: Some(Checksum { algorithm: String::from("sha256"), value: String::from("e5d3052bdaa654391c5c706d6250cea9284d3810ff3af9b359ae986cd571d3d4")}),
            location: Location { href: String::from("repodata/54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3-primary.xml.gz"),},
            timestamp: 1668072518,
            size: 17845585,
            open_size: Some(162151977),
            header_checksum: Some(Checksum { algorithm: String::from("sha256"), value: String::from("b8377a36221772919bf93f8ab4ffac46ce61d684ea7f07954455270fd291583c")}),
            database_version: Some(10),
            header_size: Some(537811),
        };
        let original_data = r##"<data type="primary">
  <checksum type="sha256">54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3</checksum>
  <open-checksum type="sha256">e5d3052bdaa654391c5c706d6250cea9284d3810ff3af9b359ae986cd571d3d4</open-checksum>
  <header-checksum type="sha256">b8377a36221772919bf93f8ab4ffac46ce61d684ea7f07954455270fd291583c</header-checksum>
  <location href="repodata/54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3-primary.xml.gz" />
  <timestamp>1668072518</timestamp>
  <size>17845585</size>
  <open-size>162151977</open-size>
  <database_version>10</database_version>
  <header-size>537811</header-size>
</data>"##;

        // Display pretty printed XML
        let yaserde_cfg = yaserde::ser::Config {
            perform_indent: true,
            write_document_declaration: false,
            ..Default::default()
        };

        let serialized = yaserde::ser::to_string_with_config(&master_entry, &yaserde_cfg).unwrap();
        assert_eq!(serialized, original_data);
    }

    #[test]
    fn test_repomd_single_entry() {
        let test_str = r##"<?xml version="1.0" encoding="UTF-8"?>
<repomd>
  <revision>1668072600</revision>
  <data type="primary">
    <checksum type="sha256">54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3</checksum>
    <open-checksum type="sha256">e5d3052bdaa654391c5c706d6250cea9284d3810ff3af9b359ae986cd571d3d4</open-checksum>
    <location href="repodata/54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3-primary.xml.gz" />
    <timestamp>1668072518</timestamp>
    <size>17845585</size>
    <open-size>162151977</open-size>
  </data>
</repomd>"##;

        let master_entry = Repomd {
            revision: 1668072600,
            data: vec![RepomdXmlDataEntry {
                kind: String::from("primary"),
                checksum: Checksum { algorithm: String::from("sha256"), value: String::from("54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3") },
                open_checksum: Some(Checksum { algorithm: String::from("sha256"), value: String::from("e5d3052bdaa654391c5c706d6250cea9284d3810ff3af9b359ae986cd571d3d4")}),
                location: Location { href: String::from("repodata/54bbae6e9d4cd4865a55f7558daef86574cddc5f2a4f8a0d9c74f946e1a45dd3-primary.xml.gz") },
                timestamp: 1668072518,
                size: 17845585,
                open_size: Some(162151977),
                header_checksum: None,
                database_version: None,
                header_size: None,
            }],
        };

        // Display pretty printed XML
        let yaserde_cfg = yaserde::ser::Config {
            perform_indent: true,
            write_document_declaration: true,
            ..Default::default()
        };

        let serialized = yaserde::ser::to_string_with_config(&master_entry, &yaserde_cfg).unwrap();
        assert_ne!(serialized, test_str);
    }
}
