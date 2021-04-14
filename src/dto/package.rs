use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    #[serde(rename(deserialize = "packages", serialize = "package_count"))]
    pub package_count: u64,
    #[serde(rename = "$value", default)]
    pub packages: Vec<Package>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename(deserialize = "type", serialize = "package_type"))]
    pub package_type: Option<String>,
    pub name: String,
    pub arch: String,
    pub version: PackageVersion,
    pub checksum: PackageChecksum,
    pub summary: String,
    pub description: String,
    pub url: String,
    pub time: PackageTime,
    pub size: PackageSize,
    pub location: PackageLocation,
    pub format: PackageFormat,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageVersion {
    pub epoch: u64,
    pub rel: String,
    pub ver: String,
}

impl ToString for PackageVersion {
    fn to_string(&self) -> String {
        format!("{}-{}", self.ver, self.rel)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageChecksum {
    pub pkgid: String,
    #[serde(rename(deserialize = "type", serialize = "checksum_type"))]
    pub checksum_type: String,
    #[serde(rename(deserialize = "$value", serialize = "$value"))]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageTime {
    pub build: u64,
    pub file: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageSize {
    pub archive: u64,
    pub installed: u64,
    pub package: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageLocation {
    pub href: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageFormat {
    #[serde(rename(deserialize = "rpm:license", serialize = "license"))]
    pub license: Option<String>,
    #[serde(rename(deserialize = "rpm:group", serialize = "group"))]
    pub group: Option<String>,
    #[serde(rename(deserialize = "rpm:buildhost", serialize = "build_host"))]
    pub build_host: Option<String>,
    #[serde(rename(deserialize = "rpm:sourcerpm", serialize = "source_rpm"))]
    pub source_rpm: Option<String>,
    #[serde(rename(deserialize = "rpm:header-range", serialize = "header_range"))]
    pub header_range: Option<PackageFormatHeaderRange>,
    #[serde(rename(deserialize = "rpm:provides", serialize = "provides"))]
    pub provides: Option<Vec<RpmEntry>>,
    #[serde(rename(deserialize = "rpm:requires", serialize = "requires"))]
    pub requires: Option<Vec<RpmEntry>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageFormatHeaderRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RpmEntry {
    pub epoch: u64,
    pub flags: String,
    pub name: String,
    pub rel: String,
    pub ver: String,
}
