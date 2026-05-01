use std::str::FromStr;

use crate::gdtf::{
    FixtureTypeId, Gdtf, Thumbnail, Version,
    bundle::{self, ResourceKey, YesNo},
};

pub struct GdtfBuilder {
    bundle: bundle::Bundle,
}

impl GdtfBuilder {
    pub fn new(bundle: bundle::Bundle) -> Self {
        Self { bundle }
    }

    pub fn build(self) -> Gdtf {
        let desc = self.bundle.description();
        let ft = &desc.fixture_type;

        let version = parse_version(&desc.data_version);

        let fixture_type_id = FixtureTypeId::from_str(&ft.fixture_type_id).unwrap();

        let reference_fixture_type_id =
            ft.ref_ft.as_deref().and_then(|s| FixtureTypeId::from_str(s).ok());

        let thumbnail = Thumbnail {
            resource: ResourceKey::new(ft.thumbnail.clone().unwrap_or_default()),
            offset_x: ft.thumbnail_offset_x.unwrap_or(0),
            offset_y: ft.thumbnail_offset_y.unwrap_or(0),
        };

        Gdtf {
            version,
            name: ft.name.clone(),
            short_name: ft.short_name.clone(),
            long_name: ft.long_name.clone(),
            manufacturer: ft.manufacturer.clone(),
            description: ft.description.clone(),
            fixture_type_id,
            reference_fixture_type_id,
            thumbnail,
            can_have_children: ft.can_have_children == YesNo::Yes,
            bundle: self.bundle,
        }
    }
}

fn parse_version(s: &str) -> Version {
    let mut parts = s.splitn(2, '.');
    let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    Version { major, minor }
}
