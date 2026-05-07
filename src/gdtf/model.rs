use crate::gdtf::{
    Name,
    bundle::{self, ResourceKey},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub(crate) name: Name,
    pub(crate) length: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) primitive_type: PrimitiveType,
    pub(crate) files: Vec<ResourceKey>,
    pub(crate) svg_offset: glam::Vec2,
    pub(crate) svg_side_offset: glam::Vec2,
    pub(crate) svg_front_offset: glam::Vec2,
}

impl Model {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn primitive_type(&self) -> &PrimitiveType {
        &self.primitive_type
    }

    pub fn files(&self) -> &[ResourceKey] {
        &self.files
    }

    pub fn svg_offset(&self) -> glam::Vec2 {
        self.svg_offset
    }

    pub fn svg_side_offset(&self) -> glam::Vec2 {
        self.svg_side_offset
    }

    pub fn svg_front_offset(&self) -> glam::Vec2 {
        self.svg_front_offset
    }
}

impl From<&bundle::Model> for Model {
    fn from(value: &bundle::Model) -> Self {
        // FIXME: Get all LODs of models from bundle.
        let files = Vec::new();

        Self {
            name: Name::new(value.name.to_owned()),
            length: value.length,
            width: value.width,
            height: value.height,
            primitive_type: (&value.primitive_type).into(),
            files,
            svg_offset: glam::Vec2::new(
                value.svg_offset_x.unwrap_or(0.0),
                value.svg_offset_y.unwrap_or(0.0),
            ),
            svg_side_offset: glam::Vec2::new(
                value.svg_offset_x.unwrap_or(0.0),
                value.svg_offset_y.unwrap_or(0.0),
            ),
            svg_front_offset: glam::Vec2::new(
                value.svg_offset_x.unwrap_or(0.0),
                value.svg_offset_y.unwrap_or(0.0),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Undefined,
    Cube,
    Cylinder,
    Sphere,
    Base,
    Yoke,
    Head,
    Scanner,
    Conventional,
    Pigtail,
    Base11,
    Scanner11,
    Conventional11,
}

impl From<&bundle::PrimitiveType> for PrimitiveType {
    fn from(value: &bundle::PrimitiveType) -> Self {
        match value {
            bundle::PrimitiveType::Undefined => PrimitiveType::Undefined,
            bundle::PrimitiveType::Cube => PrimitiveType::Cube,
            bundle::PrimitiveType::Cylinder => PrimitiveType::Cylinder,
            bundle::PrimitiveType::Sphere => PrimitiveType::Sphere,
            bundle::PrimitiveType::Base => PrimitiveType::Base,
            bundle::PrimitiveType::Yoke => PrimitiveType::Yoke,
            bundle::PrimitiveType::Head => PrimitiveType::Head,
            bundle::PrimitiveType::Scanner => PrimitiveType::Scanner,
            bundle::PrimitiveType::Conventional => PrimitiveType::Conventional,
            bundle::PrimitiveType::Pigtail => PrimitiveType::Pigtail,
            bundle::PrimitiveType::Base11 => PrimitiveType::Base11,
            bundle::PrimitiveType::Scanner11 => PrimitiveType::Scanner11,
            bundle::PrimitiveType::Conventional11 => PrimitiveType::Conventional11,
        }
    }
}
