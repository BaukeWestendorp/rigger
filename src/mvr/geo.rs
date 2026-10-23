use crate::{
    math::{self, Mat4},
    mvr::{ResourceKey, bundle},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub(in crate::mvr) local_transform: Mat4,

    pub(in crate::mvr) model: ResourceKey,
}

impl Geometry {
    pub fn local_transform(&self) -> Mat4 {
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
            local_transform: math::parse_mat4_or_identity(source.matrix.as_deref()),
            model: ResourceKey::new(&source.file_name),
        }
    }
}
