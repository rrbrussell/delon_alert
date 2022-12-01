pub struct Repomd {
    revision: String,
    data: Vec<RepomdXmlDataEntry>,
}

pub struct RepomdXmlDataEntry {
    kind: String,
    checksum: String,
    open_cheksum: String,
    location: String,
    timestamp: String,
    size: String,
    open_size: String,
}
