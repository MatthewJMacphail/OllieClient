//! responsible for parsing mojang/mod loader/mod platform metadata
use serde::Deserialize;
use time::OffsetDateTime;
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum ManifestEntryType {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha,
}

#[derive(Deserialize, Debug)]
pub struct ManifestEntry {
    #[serde(rename = "id")]
    pub minecraft_version: String,
    #[serde(rename = "type")]
    pub type_of_release: ManifestEntryType,
    #[serde(rename = "releaseTime", with = "time::serde::rfc3339")]
    pub release_time: OffsetDateTime,
    #[serde(rename = "url")]
    pub version_json_url: String,
    pub sha1: String,
}
#[derive(Deserialize, Debug)]
pub struct LatestVersions {
    #[serde(rename = "release")]
    pub release_id: String,
    #[serde(rename = "snapshot")]
    pub snapshot_id: String,
}
#[derive(Deserialize, Debug)]

pub struct VersionManifestV2 {
    pub latest: LatestVersions,
    pub versions: Vec<ManifestEntry>,
}
