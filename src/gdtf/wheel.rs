use std::str::{self, FromStr as _};

use crate::{
    CieColor,
    gdtf::{Name, Node, bundle, parse_optional_name},
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Wheel {
    pub(crate) name: Option<Name>,
    pub(crate) slots: Vec<WheelSlot>,
}

impl Wheel {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn slots(&self) -> &[WheelSlot] {
        &self.slots
    }

    pub fn slot(&self, name: &str) -> Option<&WheelSlot> {
        self.slots.iter().find(|slot| slot.name().as_str() == name)
    }
}

impl From<&bundle::Wheel> for Wheel {
    fn from(value: &bundle::Wheel) -> Self {
        Self {
            name: parse_optional_name(value.name.as_deref()),
            slots: value.slots.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WheelSlot {
    pub(crate) name: Name,
    pub(crate) color: SlotColor,
    pub(crate) file: Option<bundle::ResourceKey>,
    pub(crate) content: Option<WheelSlotContent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlotColor {
    Filter(Node),
    Cie(CieColor),
}

impl WheelSlot {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn color(&self) -> &SlotColor {
        &self.color
    }

    pub fn file(&self) -> Option<&bundle::ResourceKey> {
        self.file.as_ref()
    }

    pub fn content(&self) -> Option<&WheelSlotContent> {
        self.content.as_ref()
    }
}

impl From<&bundle::Slot> for WheelSlot {
    fn from(value: &bundle::Slot) -> Self {
        let facets: Vec<_> = value
            .content
            .iter()
            .filter_map(|c| if let bundle::SlotContent::Facet(f) = c { Some(f) } else { None })
            .collect();

        let animation = value.content.iter().find_map(|c| {
            if let bundle::SlotContent::AnimationSystem(a) = c { Some(a) } else { None }
        });

        let content = if !facets.is_empty() {
            Some(WheelSlotContent::Prism(facets.into_iter().map(|f| f.into()).collect()))
        } else if let Some(anim) = animation {
            Some(WheelSlotContent::AnimationSystem(anim.into()))
        } else {
            None
        };

        let file = if value.media_file_name.is_empty() {
            None
        } else {
            // FIXME: This only gets the file name. The key should contain the path and extension too. This
            // is not directly possible to do without a reference to the bundle's resources, so this would
            // have to be moved out of a From impl.
            // (Or we could From<(&Bundle, &bundle::Slot)> but that is cursed as fuck)
            Some(bundle::ResourceKey::new(value.media_file_name.clone()))
        };

        let color = if let Some(f) = value.filter.as_deref().filter(|f| !f.is_empty()) {
            SlotColor::Filter(Node::from_str(f).unwrap())
        } else {
            SlotColor::Cie(
                value.color.as_deref().and_then(|c| CieColor::from_str(c).ok()).unwrap_or_default(),
            )
        };

        Self { name: Name::new(value.name.clone()), color, file, content }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WheelSlotContent {
    Prism(Vec<PrismFacet>),
    AnimationSystem(AnimationSystem),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrismFacet {
    pub(crate) color: CieColor,
    pub(crate) rotation: glam::Mat3,
}

impl PrismFacet {
    pub fn color(&self) -> CieColor {
        self.color
    }

    pub fn rotation(&self) -> &glam::Mat3 {
        &self.rotation
    }
}

impl From<&bundle::Facet> for PrismFacet {
    fn from(value: &bundle::Facet) -> Self {
        Self {
            color: value
                .color
                .as_deref()
                .and_then(|c| CieColor::from_str(c).ok())
                .unwrap_or_default(),
            rotation: util::parse_mat3(&value.rotation),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationSystem {
    pub(crate) p1: glam::Vec2,
    pub(crate) p2: glam::Vec2,
    pub(crate) p3: glam::Vec2,
    pub(crate) radius: f32,
}

impl AnimationSystem {
    pub fn p1(&self) -> glam::Vec2 {
        self.p1
    }

    pub fn p2(&self) -> glam::Vec2 {
        self.p2
    }

    pub fn p3(&self) -> glam::Vec2 {
        self.p3
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl From<&bundle::AnimationSystem> for AnimationSystem {
    fn from(value: &bundle::AnimationSystem) -> Self {
        Self {
            p1: util::parse_vec2(&value.p1),
            p2: util::parse_vec2(&value.p2),
            p3: util::parse_vec2(&value.p3),
            radius: value.radius,
        }
    }
}
