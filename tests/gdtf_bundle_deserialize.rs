use std::path::Path;

use rigger::gdtf::Gdtf;

fn load_complete_gdtf() -> Gdtf {
    Gdtf::from_folder(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("gdtf")
            .join("Rigger@Complete@v1"),
    )
}

#[test]
fn test_gdtf_bundle_description() {
    let mvr = load_complete_gdtf();

    let desc = mvr.bundle().description();

    assert_eq!(desc.data_version, "1.2");
}

#[test]
fn test_gdtf_version() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.version().major(), 1);
    assert_eq!(gdtf.version().minor(), 2);
}

#[test]
fn test_gdtf_name() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.name(), "Name");
}

#[test]
fn test_gdtf_short_name() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.short_name(), Some("Short Name"));
}

#[test]
fn test_gdtf_long_name() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.long_name(), Some("Long Name"));
}

#[test]
fn test_gdtf_manufacturer() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.manufacturer(), "Manufacturer");
}

#[test]
fn test_gdtf_fixture_type_id() {
    let gdtf = load_complete_gdtf();
    assert_eq!(
        gdtf.fixture_type_id().as_uuid().to_string(),
        "ab128988-6cf0-4a87-93de-e0b2d6c7aa19"
    );
}

#[test]
fn test_gdtf_reference_fixture_type_id() {
    let gdtf = load_complete_gdtf();
    assert_eq!(
        gdtf.reference_fixture_type_id().map(|id| id.as_uuid().to_string()).as_deref(),
        Some("f0a9b846-1051-4016-a054-b1d4ff90539e")
    );
}

#[test]
fn test_gdtf_thumbnail() {
    let gdtf = load_complete_gdtf();
    let thumb = gdtf.thumbnail();
    assert_eq!(thumb.resource().as_str(), "thumbnail");
    assert_eq!(thumb.offset_x(), 197);
    assert_eq!(thumb.offset_y(), 142);
}

#[test]
fn test_gdtf_can_have_children() {
    let gdtf = load_complete_gdtf();
    assert!(gdtf.can_have_children());
}

#[test]
fn test_gdtf_bundle_from_archive() {
    let archive_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("gdtf")
        .join("Rigger@Complete@v1.gdtf");

    let gdtf = Gdtf::from_archive(archive_path);
    let desc = gdtf.bundle().description();

    assert_eq!(desc.data_version, "1.2");
}
