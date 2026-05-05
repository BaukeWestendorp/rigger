use std::{
    collections::HashMap,
    str::{self, FromStr},
};

use crate::{CieColor, gdtf::bundle, util};

use super::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Wheel {
    pub(crate) name: String,
    pub(crate) slots: HashMap<String, WheelSlot>,
}

impl Wheel {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slots(&self) -> impl Iterator<Item = &WheelSlot> {
        self.slots.values()
    }

    pub fn slot(&self, name: &str) -> Option<&WheelSlot> {
        self.slots.get(name)
    }
}

impl From<bundle::Wheel> for Wheel {
    fn from(value: bundle::Wheel) -> Self {
        Self {
            name: value.name.clone().unwrap_or_default(),
            slots: value
                .slots
                .into_iter()
                .map(|s| {
                    let slot: WheelSlot = s.into();
                    (slot.name.to_string(), slot)
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WheelSlot {
    pub(crate) name: String,
    pub(crate) color: SlotColor,
    pub(crate) media_file: Option<bundle::ResourceKey>,
    pub(crate) content: Option<WheelSlotContent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlotColor {
    Filter(Node),
    Cie(CieColor),
}

impl WheelSlot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> &SlotColor {
        &self.color
    }

    pub fn media_file(&self) -> Option<&bundle::ResourceKey> {
        self.media_file.as_ref()
    }

    pub fn content(&self) -> Option<&WheelSlotContent> {
        self.content.as_ref()
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

impl From<bundle::Slot> for WheelSlot {
    fn from(value: bundle::Slot) -> Self {
        (&value).into()
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
            Some(WheelSlotContent::Prism(facets.iter().map(|f| (*f).into()).collect()))
        } else if let Some(anim) = animation {
            Some(WheelSlotContent::AnimationSystem(anim.into()))
        } else {
            None
        };

        let media_file_name = if value.media_file_name.is_empty() {
            None
        } else {
            Some(value.media_file_name.clone())
        };

        let color = if let Some(f) = value.filter.as_deref().filter(|f| !f.is_empty()) {
            SlotColor::Filter(Node::from_str(f).unwrap())
        } else {
            SlotColor::Cie(
                value.color.as_deref().and_then(|c| CieColor::from_str(c).ok()).unwrap_or_default(),
            )
        };

        Self {
            name: value.name.clone(),
            color,
            media_file: media_file_name.map(|mfn| bundle::ResourceKey::new(mfn)),
            content,
        }
    }
}
