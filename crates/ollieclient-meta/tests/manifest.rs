#![expect(
    clippy::expect_used,
    reason = "Tests should panic when its fixture is broken"
)]
use ollieclient_meta::{ManifestEntryType, VersionManifestV2};

fn load_manifest() -> VersionManifestV2 {
    let json = include_str!("fixtures/version_manifest_v2.json");
    serde_json::from_str(json).expect("fixture should parse")
}
#[test]
fn latest_release_id_is_correct() {
    let manifest = load_manifest();

    // Assert: check the result makes sense
    assert_eq!(manifest.latest.release_id, "26.3");
}

#[test]
fn latest_release_is_in_versions_list() {
    let manifest = load_manifest();
    assert!(
        manifest
            .versions
            .iter()
            .any(|entry| entry.minecraft_version == manifest.latest.release_id)
    );
}
#[test]
fn latest_release_is_in_versions_list_and_is_correct_type() {
    let manifest = load_manifest();
    assert!(manifest.versions.iter().any(|entry| entry.minecraft_version
        == manifest.latest.release_id
        && entry.type_of_release == ManifestEntryType::Release));
}
#[test]
fn latest_snapshot_is_in_versions_list() {
    let manifest = load_manifest();
    assert!(
        manifest
            .versions
            .iter()
            .any(|entry| entry.minecraft_version == manifest.latest.snapshot_id)
    );
}

#[test]
fn latest_snapshot_is_in_versions_list_and_is_correct_type() {
    let manifest = load_manifest();
    assert!(manifest.versions.iter().any(|entry| entry.minecraft_version
        == manifest.latest.snapshot_id
        && entry.type_of_release == ManifestEntryType::Snapshot));
}
