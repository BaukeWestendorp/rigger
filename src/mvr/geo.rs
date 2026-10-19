use crate::{
    math::{self, Transform},
    mvr::{ResourceKey, bundle},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub(in crate::mvr) local_transform: Transform,

    pub(in crate::mvr) model: ResourceKey,
}

impl Geometry {
    pub fn local_transform(&self) -> Transform {
        self.local_transform
    }

    pub fn model(&self) -> &ResourceKey {
        &self.model
    }
}

impl bundle::FromBundle for Geometry {
    type Source = bundle::Geometry3D;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        Geometry {
            local_transform: math::parse_transform_or_identity(source.matrix.as_deref()),
            model: ResourceKey::new(&source.file_name),
        }
    }
}
