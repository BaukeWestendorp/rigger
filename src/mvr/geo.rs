use crate::mvr::bundle::ResourceKey;

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub(crate) local_transform: glam::Affine3A,

    pub(crate) model: ResourceKey,
}

impl Geometry {
    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn model(&self) -> &ResourceKey {
        &self.model
    }
}
