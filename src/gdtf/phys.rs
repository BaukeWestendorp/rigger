use std::str::FromStr as _;

use crate::{
    CieColor,
    gdtf::{Name, bundle, parse_optional_name},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Emitter {
    pub(crate) name: Name,
    pub(crate) color: EmitterColor,
    pub(crate) diode_part: Option<String>,
    pub(crate) measurements: Vec<EmitterMeasurement>,
}

impl Emitter {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn color(&self) -> EmitterColor {
        self.color
    }

    pub fn diode_part(&self) -> Option<&str> {
        self.diode_part.as_deref()
    }

    pub fn measurements(&self) -> &[EmitterMeasurement] {
        &self.measurements
    }
}

impl From<&bundle::Emitter> for Emitter {
    fn from(value: &bundle::Emitter) -> Self {
        let color = if let Some(c) = value.color.as_deref() {
            EmitterColor::Color(CieColor::from_str(c).unwrap())
        } else {
            EmitterColor::DominantWaveLength(value.dominant_wave_length.unwrap())
        };

        Self {
            name: Name::new(value.name.clone()),
            color,
            diode_part: value.diode_part.clone(),
            measurements: value.measurements.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmitterColor {
    Color(CieColor),
    DominantWaveLength(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmitterMeasurement {
    pub(crate) physical: f32,
    pub(crate) luminous_intensity: f32,
    pub(crate) interpolation_to: InterpolationTo,
    pub(crate) points: Vec<MeasurementPoint>,
}

impl EmitterMeasurement {
    pub fn physical(&self) -> f32 {
        self.physical
    }

    pub fn luminous_intensity(&self) -> f32 {
        self.luminous_intensity
    }

    pub fn interpolation_to(&self) -> InterpolationTo {
        self.interpolation_to
    }

    pub fn points(&self) -> &[MeasurementPoint] {
        &self.points
    }
}

impl From<&bundle::EmitterMeasurement> for EmitterMeasurement {
    fn from(value: &bundle::EmitterMeasurement) -> Self {
        Self {
            physical: value.physical,
            luminous_intensity: value.luminous_intensity,
            interpolation_to: (&value.interpolation_to).into(),
            points: value.measurement_points.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    pub(crate) name: Name,
    pub(crate) color: CieColor,
    pub(crate) measurements: Vec<FilterMeasurement>,
}

impl Filter {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn color(&self) -> CieColor {
        self.color
    }

    pub fn measurements(&self) -> &[FilterMeasurement] {
        &self.measurements
    }
}

impl From<&bundle::Filter> for Filter {
    fn from(value: &bundle::Filter) -> Self {
        Self {
            name: Name::new(value.name.clone()),
            color: value
                .color
                .as_deref()
                .map(|c| CieColor::from_str(c).unwrap())
                .unwrap_or_default(),
            measurements: value.measurements.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterMeasurement {
    pub(crate) physical: f32,
    pub(crate) transmission: f32,
    pub(crate) interpolation_to: InterpolationTo,
    pub(crate) points: Vec<MeasurementPoint>,
}

impl FilterMeasurement {
    pub fn physical(&self) -> f32 {
        self.physical
    }

    pub fn transmission(&self) -> f32 {
        self.transmission
    }

    pub fn interpolation_to(&self) -> InterpolationTo {
        self.interpolation_to
    }

    pub fn points(&self) -> &[MeasurementPoint] {
        &self.points
    }
}

impl From<&bundle::FilterMeasurement> for FilterMeasurement {
    fn from(value: &bundle::FilterMeasurement) -> Self {
        Self {
            physical: value.physical,
            transmission: value.transmission,
            interpolation_to: (&value.interpolation_to).into(),
            points: value.measurement_points.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasurementPoint {
    pub(crate) wave_length: f32,
    pub(crate) energy: f32,
}

impl MeasurementPoint {
    pub fn wave_length(&self) -> f32 {
        self.wave_length
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }
}

impl From<&bundle::MeasurementPoint> for MeasurementPoint {
    fn from(value: &bundle::MeasurementPoint) -> Self {
        Self { wave_length: value.wave_length, energy: value.energy }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationTo {
    Linear,
    Step,
    Log,
}

impl From<&bundle::InterpolationTo> for InterpolationTo {
    fn from(value: &bundle::InterpolationTo) -> Self {
        match value {
            bundle::InterpolationTo::Linear => Self::Linear,
            bundle::InterpolationTo::Step => Self::Step,
            bundle::InterpolationTo::Log => Self::Log,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorSpace {
    pub(crate) name: Option<Name>,
    pub(crate) mode: ColorSpaceMode,
}

impl ColorSpace {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn mode(&self) -> &ColorSpaceMode {
        &self.mode
    }
}

impl From<&bundle::ColorSpace> for ColorSpace {
    fn from(value: &bundle::ColorSpace) -> Self {
        Self { name: parse_optional_name(value.name.as_deref()), mode: value.into() }
    }
}

// FIXME: Make red, green, blue and white_point values available with
// getters for SRgb, ProPhoto and Ansi too as seen in Table 21 of GDTF.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorSpaceMode {
    SRgb,
    ProPhoto,
    Ansi,
    Custom { red: CieColor, green: CieColor, blue: CieColor, white_point: CieColor },
}

impl From<&bundle::ColorSpace> for ColorSpaceMode {
    fn from(value: &bundle::ColorSpace) -> Self {
        match value.mode {
            bundle::ColorSpaceMode::Custom => Self::Custom {
                red: CieColor::from_str(value.red.as_deref().unwrap()).unwrap(),
                green: CieColor::from_str(value.green.as_deref().unwrap()).unwrap(),
                blue: CieColor::from_str(value.blue.as_deref().unwrap()).unwrap(),
                white_point: CieColor::from_str(value.white_point.as_deref().unwrap()).unwrap(),
            },
            bundle::ColorSpaceMode::SRgb => Self::SRgb,
            bundle::ColorSpaceMode::ProPhoto => Self::ProPhoto,
            bundle::ColorSpaceMode::Ansi => Self::Ansi,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gamut {
    pub(crate) name: Option<Name>,
    pub(crate) points: Vec<CieColor>,
}

impl Gamut {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn points(&self) -> &[CieColor] {
        &self.points
    }
}

impl From<&bundle::Gamut> for Gamut {
    fn from(value: &bundle::Gamut) -> Self {
        let points = value.points.as_deref().map(parse_gamut_points).unwrap_or_default();
        Self { name: parse_optional_name(value.name.as_deref()), points }
    }
}

fn parse_gamut_points(s: &str) -> Vec<CieColor> {
    s.split("},{")
        .map(|chunk| {
            let trimmed = chunk.trim_matches(|c| c == '{' || c == '}');
            CieColor::from_str(trimmed).unwrap()
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxProfile {
    pub(crate) name: Option<Name>,
    pub(crate) points: Vec<DmxPoint>,
}

impl DmxProfile {
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn points(&self) -> &[DmxPoint] {
        &self.points
    }
}

impl From<&bundle::DmxProfile> for DmxProfile {
    fn from(value: &bundle::DmxProfile) -> Self {
        Self {
            name: parse_optional_name(value.name.as_deref()),
            points: value.points.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DmxPoint {
    pub(crate) dmx_percentage: f32,
    pub(crate) cfc: [f32; 4],
}

impl DmxPoint {
    pub fn dmx_percentage(&self) -> f32 {
        self.dmx_percentage
    }

    pub fn cfc(&self) -> [f32; 4] {
        self.cfc
    }
}

impl From<&bundle::Point> for DmxPoint {
    fn from(value: &bundle::Point) -> Self {
        Self {
            dmx_percentage: value.dmx_percentage.unwrap_or(0.0),
            cfc: [
                value.cfc_0.unwrap_or(0.0),
                value.cfc_1.unwrap_or(0.0),
                value.cfc_2.unwrap_or(0.0),
                value.cfc_3.unwrap_or(0.0),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CriGroup {
    pub(crate) color_temperature: f32,
    pub(crate) cris: Vec<Cri>,
}

impl CriGroup {
    pub fn color_temperature(&self) -> f32 {
        self.color_temperature
    }

    pub fn cris(&self) -> &[Cri] {
        &self.cris
    }
}

impl From<&bundle::CriGroup> for CriGroup {
    fn from(value: &bundle::CriGroup) -> Self {
        Self {
            color_temperature: value.color_temperature.unwrap_or(6000.0),
            cris: value.cris.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cri {
    pub(crate) ces: Ces,
    pub(crate) color_rendering_index: u8,
}

impl Cri {
    pub fn ces(&self) -> Ces {
        self.ces.clone()
    }

    pub fn color_rendering_index(&self) -> u8 {
        self.color_rendering_index
    }
}

impl From<&bundle::Cri> for Cri {
    fn from(value: &bundle::Cri) -> Self {
        Self {
            ces: (&value.ces).into(),
            color_rendering_index: value.color_rendering_index.unwrap_or(100),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Properties {
    pub(crate) operating_temperature: Option<OperatingTemperature>,
    pub(crate) weight: Option<f32>,
    pub(crate) leg_height: Option<f32>,
}

impl Properties {
    pub fn operating_temperature(&self) -> Option<&OperatingTemperature> {
        self.operating_temperature.as_ref()
    }

    pub fn weight(&self) -> Option<f32> {
        self.weight
    }

    pub fn leg_height(&self) -> Option<f32> {
        self.leg_height
    }
}

impl From<&bundle::Properties> for Properties {
    fn from(value: &bundle::Properties) -> Self {
        let mut operating_temperature = None;
        let mut weight = None;
        let mut leg_height = None;

        for item in &value.content {
            match item {
                bundle::PropertiesContent::OperatingTemperature(ot) => {
                    operating_temperature = Some(ot.into());
                }
                bundle::PropertiesContent::Weight(w) => {
                    weight = w.value;
                }
                bundle::PropertiesContent::LegHeight(lh) => {
                    leg_height = lh.value;
                }
                bundle::PropertiesContent::PowerConsumption(_) => {}
            }
        }

        Self { operating_temperature, weight, leg_height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperatingTemperature {
    pub(crate) low: f32,
    pub(crate) high: f32,
}

impl OperatingTemperature {
    pub fn low(&self) -> f32 {
        self.low
    }

    pub fn high(&self) -> f32 {
        self.high
    }
}

impl From<&bundle::OperatingTemperature> for OperatingTemperature {
    fn from(value: &bundle::OperatingTemperature) -> Self {
        Self { low: value.low.unwrap_or(0.0), high: value.high.unwrap_or(40.0) }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ces {
    Ces01, Ces02, Ces03, Ces04, Ces05, Ces06, Ces07, Ces08, Ces09, Ces10,
    Ces11, Ces12, Ces13, Ces14, Ces15, Ces16, Ces17, Ces18, Ces19, Ces20,
    Ces21, Ces22, Ces23, Ces24, Ces25, Ces26, Ces27, Ces28, Ces29, Ces30,
    Ces31, Ces32, Ces33, Ces34, Ces35, Ces36, Ces37, Ces38, Ces39, Ces40,
    Ces41, Ces42, Ces43, Ces44, Ces45, Ces46, Ces47, Ces48, Ces49, Ces50,
    Ces51, Ces52, Ces53, Ces54, Ces55, Ces56, Ces57, Ces58, Ces59, Ces60,
    Ces61, Ces62, Ces63, Ces64, Ces65, Ces66, Ces67, Ces68, Ces69, Ces70,
    Ces71, Ces72, Ces73, Ces74, Ces75, Ces76, Ces77, Ces78, Ces79, Ces80,
    Ces81, Ces82, Ces83, Ces84, Ces85, Ces86, Ces87, Ces88, Ces89, Ces90,
    Ces91, Ces92, Ces93, Ces94, Ces95, Ces96, Ces97, Ces98, Ces99,
}

impl From<&bundle::Ces> for Ces {
    fn from(value: &bundle::Ces) -> Self {
        match value {
            bundle::Ces::Ces01 => Ces::Ces01,
            bundle::Ces::Ces02 => Ces::Ces02,
            bundle::Ces::Ces03 => Ces::Ces03,
            bundle::Ces::Ces04 => Ces::Ces04,
            bundle::Ces::Ces05 => Ces::Ces05,
            bundle::Ces::Ces06 => Ces::Ces06,
            bundle::Ces::Ces07 => Ces::Ces07,
            bundle::Ces::Ces08 => Ces::Ces08,
            bundle::Ces::Ces09 => Ces::Ces09,
            bundle::Ces::Ces10 => Ces::Ces10,
            bundle::Ces::Ces11 => Ces::Ces11,
            bundle::Ces::Ces12 => Ces::Ces12,
            bundle::Ces::Ces13 => Ces::Ces13,
            bundle::Ces::Ces14 => Ces::Ces14,
            bundle::Ces::Ces15 => Ces::Ces15,
            bundle::Ces::Ces16 => Ces::Ces16,
            bundle::Ces::Ces17 => Ces::Ces17,
            bundle::Ces::Ces18 => Ces::Ces18,
            bundle::Ces::Ces19 => Ces::Ces19,
            bundle::Ces::Ces20 => Ces::Ces20,
            bundle::Ces::Ces21 => Ces::Ces21,
            bundle::Ces::Ces22 => Ces::Ces22,
            bundle::Ces::Ces23 => Ces::Ces23,
            bundle::Ces::Ces24 => Ces::Ces24,
            bundle::Ces::Ces25 => Ces::Ces25,
            bundle::Ces::Ces26 => Ces::Ces26,
            bundle::Ces::Ces27 => Ces::Ces27,
            bundle::Ces::Ces28 => Ces::Ces28,
            bundle::Ces::Ces29 => Ces::Ces29,
            bundle::Ces::Ces30 => Ces::Ces30,
            bundle::Ces::Ces31 => Ces::Ces31,
            bundle::Ces::Ces32 => Ces::Ces32,
            bundle::Ces::Ces33 => Ces::Ces33,
            bundle::Ces::Ces34 => Ces::Ces34,
            bundle::Ces::Ces35 => Ces::Ces35,
            bundle::Ces::Ces36 => Ces::Ces36,
            bundle::Ces::Ces37 => Ces::Ces37,
            bundle::Ces::Ces38 => Ces::Ces38,
            bundle::Ces::Ces39 => Ces::Ces39,
            bundle::Ces::Ces40 => Ces::Ces40,
            bundle::Ces::Ces41 => Ces::Ces41,
            bundle::Ces::Ces42 => Ces::Ces42,
            bundle::Ces::Ces43 => Ces::Ces43,
            bundle::Ces::Ces44 => Ces::Ces44,
            bundle::Ces::Ces45 => Ces::Ces45,
            bundle::Ces::Ces46 => Ces::Ces46,
            bundle::Ces::Ces47 => Ces::Ces47,
            bundle::Ces::Ces48 => Ces::Ces48,
            bundle::Ces::Ces49 => Ces::Ces49,
            bundle::Ces::Ces50 => Ces::Ces50,
            bundle::Ces::Ces51 => Ces::Ces51,
            bundle::Ces::Ces52 => Ces::Ces52,
            bundle::Ces::Ces53 => Ces::Ces53,
            bundle::Ces::Ces54 => Ces::Ces54,
            bundle::Ces::Ces55 => Ces::Ces55,
            bundle::Ces::Ces56 => Ces::Ces56,
            bundle::Ces::Ces57 => Ces::Ces57,
            bundle::Ces::Ces58 => Ces::Ces58,
            bundle::Ces::Ces59 => Ces::Ces59,
            bundle::Ces::Ces60 => Ces::Ces60,
            bundle::Ces::Ces61 => Ces::Ces61,
            bundle::Ces::Ces62 => Ces::Ces62,
            bundle::Ces::Ces63 => Ces::Ces63,
            bundle::Ces::Ces64 => Ces::Ces64,
            bundle::Ces::Ces65 => Ces::Ces65,
            bundle::Ces::Ces66 => Ces::Ces66,
            bundle::Ces::Ces67 => Ces::Ces67,
            bundle::Ces::Ces68 => Ces::Ces68,
            bundle::Ces::Ces69 => Ces::Ces69,
            bundle::Ces::Ces70 => Ces::Ces70,
            bundle::Ces::Ces71 => Ces::Ces71,
            bundle::Ces::Ces72 => Ces::Ces72,
            bundle::Ces::Ces73 => Ces::Ces73,
            bundle::Ces::Ces74 => Ces::Ces74,
            bundle::Ces::Ces75 => Ces::Ces75,
            bundle::Ces::Ces76 => Ces::Ces76,
            bundle::Ces::Ces77 => Ces::Ces77,
            bundle::Ces::Ces78 => Ces::Ces78,
            bundle::Ces::Ces79 => Ces::Ces79,
            bundle::Ces::Ces80 => Ces::Ces80,
            bundle::Ces::Ces81 => Ces::Ces81,
            bundle::Ces::Ces82 => Ces::Ces82,
            bundle::Ces::Ces83 => Ces::Ces83,
            bundle::Ces::Ces84 => Ces::Ces84,
            bundle::Ces::Ces85 => Ces::Ces85,
            bundle::Ces::Ces86 => Ces::Ces86,
            bundle::Ces::Ces87 => Ces::Ces87,
            bundle::Ces::Ces88 => Ces::Ces88,
            bundle::Ces::Ces89 => Ces::Ces89,
            bundle::Ces::Ces90 => Ces::Ces90,
            bundle::Ces::Ces91 => Ces::Ces91,
            bundle::Ces::Ces92 => Ces::Ces92,
            bundle::Ces::Ces93 => Ces::Ces93,
            bundle::Ces::Ces94 => Ces::Ces94,
            bundle::Ces::Ces95 => Ces::Ces95,
            bundle::Ces::Ces96 => Ces::Ces96,
            bundle::Ces::Ces97 => Ces::Ces97,
            bundle::Ces::Ces98 => Ces::Ces98,
            bundle::Ces::Ces99 => Ces::Ces99,
        }
    }
}
