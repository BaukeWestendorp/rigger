use std::{path::Path, str::FromStr as _};

use rigger::{
    CieColor,
    gdtf::{
        Gdtf, Name, NodePath,
        attr::{ActivationGroup, AttributeName, PhysicalUnit, SubPhysicalUnitType},
        phys::{Ces, ColorSpaceMode, EmitterColor, InterpolationTo},
        wheel::{SlotColor, WheelSlotContent},
    },
};
use uuid::Uuid;

fn load_complete_gdtf() -> Gdtf {
    Gdtf::from_folder(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("gdtf")
            .join("Rigger@Complete@v1"),
    )
}

#[test]
fn test_gdtf_version() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.version().major(), 1);
    assert_eq!(gdtf.version().minor(), 2);
}

#[test]
fn test_gdtf_basic_info() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.name(), "Name");
    assert_eq!(gdtf.long_name(), Some("Long Name"));
    assert_eq!(gdtf.short_name(), Some("Short Name"));
    assert_eq!(gdtf.manufacturer(), "Rigger");
    assert_eq!(gdtf.description(), "Description");
}

#[test]
fn test_gdtf_fixture_type_ids() {
    let gdtf = load_complete_gdtf();
    assert_eq!(
        gdtf.fixture_type_id().as_uuid(),
        Uuid::from_str("ab128988-6cf0-4a87-93de-e0b2d6c7aa19").unwrap()
    );
    assert_eq!(
        gdtf.reference_fixture_type_id().unwrap().as_uuid(),
        Uuid::from_str("f0a9b846-1051-4016-a054-b1d4ff90539e").unwrap()
    );
}

#[test]
fn test_gdtf_thumbnail() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.thumbnail_offset().x, 197);
    assert_eq!(gdtf.thumbnail_offset().y, 142);
    assert!(gdtf.resources().thumbnail_png().is_some());
    assert!(gdtf.resources().thumbnail_svg().is_some());
}

#[test]
fn test_gdtf_can_have_children() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.can_have_children(), true);
}

#[test]
fn test_gdtf_feature_groups() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.feature_groups().len(), 7);

    let position = gdtf.feature_group(&Name::new("Position")).unwrap();
    assert_eq!(position.name().as_str(), "Position");
    assert_eq!(position.pretty_name(), "Position");
    assert_eq!(position.features().len(), 1);
    assert_eq!(position.feature(&Name::new("PanTilt")).unwrap().name().as_str(), "PanTilt");

    let dimmer = gdtf.feature_group(&Name::new("Dimmer")).unwrap();
    assert_eq!(dimmer.name().as_str(), "Dimmer");
    assert_eq!(dimmer.pretty_name(), "Dimmer");
    assert_eq!(dimmer.features().len(), 1);
    assert_eq!(dimmer.feature(&Name::new("Dimmer")).unwrap().name().as_str(), "Dimmer");

    let color = gdtf.feature_group(&Name::new("Color")).unwrap();
    assert_eq!(color.name().as_str(), "Color");
    assert_eq!(color.pretty_name(), "Color");
    assert_eq!(color.features().len(), 1);
    assert_eq!(color.feature(&Name::new("Color")).unwrap().name().as_str(), "Color");

    let gobo = gdtf.feature_group(&Name::new("Gobo")).unwrap();
    assert_eq!(gobo.name().as_str(), "Gobo");
    assert_eq!(gobo.pretty_name(), "Gobo");
    assert_eq!(gobo.features().len(), 1);
    assert_eq!(gobo.feature(&Name::new("Gobo")).unwrap().name().as_str(), "Gobo");

    let beam = gdtf.feature_group(&Name::new("Beam")).unwrap();
    assert_eq!(beam.name().as_str(), "Beam");
    assert_eq!(beam.pretty_name(), "Beam");
    assert_eq!(beam.features().len(), 1);
    assert_eq!(beam.feature(&Name::new("Beam")).unwrap().name().as_str(), "Beam");

    let focus = gdtf.feature_group(&Name::new("Focus")).unwrap();
    assert_eq!(focus.name().as_str(), "Focus");
    assert_eq!(focus.pretty_name(), "Focus");
    assert_eq!(focus.features().len(), 1);
    assert_eq!(focus.feature(&Name::new("Focus")).unwrap().name().as_str(), "Focus");

    let control = gdtf.feature_group(&Name::new("Control")).unwrap();
    assert_eq!(control.name().as_str(), "Control");
    assert_eq!(control.pretty_name(), "Control");
    assert_eq!(control.features().len(), 1);
    assert_eq!(control.feature(&Name::new("Control")).unwrap().name().as_str(), "Control");
}

#[test]
fn test_gdtf_activation_groups() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.activation_groups().len(), 17);
    assert_eq!(gdtf.activation_group(&Name::new("PanTilt")).unwrap(), &ActivationGroup::PanTilt);
    assert_eq!(gdtf.activation_group(&Name::new("XYZ")).unwrap(), &ActivationGroup::Xyz);
    assert_eq!(gdtf.activation_group(&Name::new("Rot_XYZ")).unwrap(), &ActivationGroup::RotXyz);
    assert_eq!(gdtf.activation_group(&Name::new("Scale_XYZ")).unwrap(), &ActivationGroup::ScaleXyz);
    assert_eq!(gdtf.activation_group(&Name::new("ColorRGB")).unwrap(), &ActivationGroup::ColorRgb);
    assert_eq!(gdtf.activation_group(&Name::new("ColorHSB")).unwrap(), &ActivationGroup::ColorHsb);
    assert_eq!(gdtf.activation_group(&Name::new("ColorCIE")).unwrap(), &ActivationGroup::ColorCie);
    assert_eq!(
        gdtf.activation_group(&Name::new("ColorIndirect")).unwrap(),
        &ActivationGroup::ColorIndirect
    );
    assert_eq!(gdtf.activation_group(&Name::new("Prism")).unwrap(), &ActivationGroup::Prism);
    assert_eq!(
        gdtf.activation_group(&Name::new("BeamShaper")).unwrap(),
        &ActivationGroup::BeamShaper
    );
    assert_eq!(gdtf.activation_group(&Name::new("Shaper")).unwrap(), &ActivationGroup::Shaper);
    assert_eq!(gdtf.activation_group(&Name::new("Gobo1")).unwrap(), &ActivationGroup::Gobo(1));
    assert_eq!(
        gdtf.activation_group(&Name::new("Gobo1Pos")).unwrap(),
        &ActivationGroup::GoboPos(1)
    );
    assert_eq!(
        gdtf.activation_group(&Name::new("AnimationWheel1")).unwrap(),
        &ActivationGroup::AnimationWheel(1)
    );
    assert_eq!(
        gdtf.activation_group(&Name::new("AnimationWheel1Pos")).unwrap(),
        &ActivationGroup::AnimationWheelPos(1)
    );
    assert_eq!(
        gdtf.activation_group(&Name::new("AnimationSystem1")).unwrap(),
        &ActivationGroup::AnimationSystem(1)
    );
    assert_eq!(
        gdtf.activation_group(&Name::new("AnimationSystem1Pos")).unwrap(),
        &ActivationGroup::AnimationSystemPos(1)
    );
}

#[test]
fn test_gdtf_attributes() {
    let gdtf = load_complete_gdtf();

    assert_eq!(gdtf.attributes().len(), 10);

    let pan = gdtf.attribute(&AttributeName::Pan).unwrap();
    assert_eq!(pan.name(), &AttributeName::Pan);
    assert_eq!(pan.pretty_name(), "Pan");
    assert_eq!(pan.activation_group(), Some(&NodePath::from_str("PanTilt").unwrap()));
    assert_eq!(pan.feature(), Some(&NodePath::from_str("Position.PanTilt").unwrap()));
    assert_eq!(pan.main_attribute(), None);
    assert_eq!(pan.physical_unit(), PhysicalUnit::Angle);
    assert_eq!(pan.color(), None);
    assert!(pan.sub_physical_units().is_empty());

    let tilt = gdtf.attribute(&AttributeName::Tilt).unwrap();
    assert_eq!(tilt.name(), &AttributeName::Tilt);
    assert_eq!(tilt.pretty_name(), "Tilt");
    assert_eq!(tilt.activation_group(), Some(&NodePath::from_str("PanTilt").unwrap()));
    assert_eq!(tilt.feature(), Some(&NodePath::from_str("Position.PanTilt").unwrap()));
    assert_eq!(tilt.main_attribute(), None);
    assert_eq!(tilt.physical_unit(), PhysicalUnit::Angle);
    assert_eq!(tilt.color(), None);
    assert!(tilt.sub_physical_units().is_empty());

    let dimmer = gdtf.attribute(&AttributeName::Dimmer).unwrap();
    assert_eq!(dimmer.name(), &AttributeName::Dimmer);
    assert_eq!(dimmer.pretty_name(), "Dimmer");
    assert_eq!(dimmer.activation_group(), None);
    assert_eq!(dimmer.feature(), Some(&NodePath::from_str("Dimmer.Dimmer").unwrap()));
    assert_eq!(dimmer.main_attribute(), None);
    assert_eq!(dimmer.physical_unit(), PhysicalUnit::LuminousIntensity);
    assert_eq!(dimmer.color(), None);
    assert!(dimmer.sub_physical_units().is_empty());

    let color_r = gdtf.attribute(&AttributeName::ColorAddR).unwrap();
    assert_eq!(color_r.name(), &AttributeName::ColorAddR);
    assert_eq!(color_r.pretty_name(), "R");
    assert_eq!(color_r.activation_group(), None);
    assert_eq!(color_r.feature(), Some(&NodePath::from_str("Color.Color").unwrap()));
    assert_eq!(color_r.main_attribute(), None);
    assert_eq!(color_r.physical_unit(), PhysicalUnit::ColorComponent);
    assert_color(color_r.color().unwrap(), CieColor::new(0.7347, 0.2653, 0.2126));
    assert!(color_r.sub_physical_units().is_empty());

    let color_g = gdtf.attribute(&AttributeName::ColorAddG).unwrap();
    assert_eq!(color_g.name(), &AttributeName::ColorAddG);
    assert_eq!(color_g.pretty_name(), "G");
    assert_eq!(color_g.activation_group(), None);
    assert_eq!(color_g.feature(), Some(&NodePath::from_str("Color.Color").unwrap()));
    assert_eq!(color_g.main_attribute(), None);
    assert_eq!(color_g.physical_unit(), PhysicalUnit::ColorComponent);
    assert_color(color_g.color().unwrap(), CieColor::new(0.1596, 0.8404, 0.7152));
    assert!(color_g.sub_physical_units().is_empty());

    let color_b = gdtf.attribute(&AttributeName::ColorAddB).unwrap();
    assert_eq!(color_b.name(), &AttributeName::ColorAddB);
    assert_eq!(color_b.pretty_name(), "B");
    assert_eq!(color_b.activation_group(), None);
    assert_eq!(color_b.feature(), Some(&NodePath::from_str("Color.Color").unwrap()));
    assert_eq!(color_b.main_attribute(), None);
    assert_eq!(color_b.physical_unit(), PhysicalUnit::ColorComponent);
    assert_color(color_b.color().unwrap(), CieColor::new(0.0366, 0.0001, 0.0722));
    assert!(color_b.sub_physical_units().is_empty());

    let gobo1 = gdtf.attribute(&AttributeName::Gobo(1)).unwrap();
    assert_eq!(gobo1.name(), &AttributeName::Gobo(1));
    assert_eq!(gobo1.pretty_name(), "Gobo 1");
    assert_eq!(gobo1.activation_group(), None);
    assert_eq!(gobo1.feature(), Some(&NodePath::from_str("Gobo.Gobo").unwrap()));
    assert_eq!(gobo1.main_attribute(), None);
    assert_eq!(gobo1.physical_unit(), PhysicalUnit::None);
    assert_eq!(gobo1.color(), None);
    assert!(gobo1.sub_physical_units().is_empty());

    let zoom = gdtf.attribute(&AttributeName::Zoom).unwrap();
    assert_eq!(zoom.name(), &AttributeName::Zoom);
    assert_eq!(zoom.pretty_name(), "Zoom");
    assert_eq!(zoom.activation_group(), None);
    assert_eq!(zoom.feature(), Some(&NodePath::from_str("Beam.Beam").unwrap()));
    assert_eq!(zoom.main_attribute(), None);
    assert_eq!(zoom.physical_unit(), PhysicalUnit::Angle);
    assert_eq!(zoom.color(), None);
    assert!(zoom.sub_physical_units().is_empty());

    let focus1 = gdtf.attribute(&AttributeName::Focus(1)).unwrap();
    assert_eq!(focus1.name(), &AttributeName::Focus(1));
    assert_eq!(focus1.pretty_name(), "Focus");
    assert_eq!(focus1.activation_group(), None);
    assert_eq!(focus1.feature(), Some(&NodePath::from_str("Focus.Focus").unwrap()));
    assert_eq!(focus1.main_attribute(), None);
    assert_eq!(focus1.physical_unit(), PhysicalUnit::Length);
    assert_eq!(focus1.color(), None);
    assert!(focus1.sub_physical_units().is_empty());

    let shutter1 = gdtf.attribute(&AttributeName::Shutter(1)).unwrap();
    assert_eq!(shutter1.name(), &AttributeName::Shutter(1));
    assert_eq!(shutter1.pretty_name(), "Shutter");
    assert_eq!(shutter1.activation_group(), None);
    assert_eq!(shutter1.feature(), Some(&NodePath::from_str("Control.Control").unwrap()));
    assert_eq!(shutter1.main_attribute(), None);
    assert_eq!(shutter1.physical_unit(), PhysicalUnit::Frequency);
    assert_eq!(shutter1.color(), None);

    let sub_units = shutter1.sub_physical_units();
    assert_eq!(sub_units.len(), 2);

    assert_eq!(sub_units[0].r#type(), SubPhysicalUnitType::Duration);
    assert_eq!(sub_units[0].physical_unit(), PhysicalUnit::Time);
    assert_eq!(sub_units[0].physical_from(), 0.0);
    assert_eq!(sub_units[0].physical_to(), 1.0);

    assert_eq!(sub_units[1].r#type(), SubPhysicalUnitType::DutyCycle);
    assert_eq!(sub_units[1].physical_unit(), PhysicalUnit::Percent);
    assert_eq!(sub_units[1].physical_from(), 0.0);
    assert_eq!(sub_units[1].physical_to(), 100.0);
}

fn assert_color(actual: CieColor, expected: CieColor) {
    assert_eq!(actual, expected);
}

#[test]
fn test_gdtf_wheel_count() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.wheels().len(), 2);
    assert!(gdtf.wheel(&Name::new("ColorWheel")).is_some());
    assert!(gdtf.wheel(&Name::new("GoboWheel")).is_some());
}

#[test]
fn test_gdtf_color_wheel_slots() {
    let gdtf = load_complete_gdtf();
    let wheel = gdtf.wheel(&Name::new("ColorWheel")).unwrap();
    assert_eq!(wheel.slots().len(), 4);

    let open = wheel.slot(&Name::new("Open")).unwrap();
    assert_eq!(open.color(), &SlotColor::Cie(CieColor::new(0.3127, 0.3290, 100.0)));
    assert_eq!(open.content(), None);
    assert_eq!(open.file(), None);

    let red = wheel.slot(&Name::new("Red")).unwrap();
    assert_eq!(red.color(), &SlotColor::Filter(NodePath::from_str("RedFilter").unwrap()));
    assert_eq!(red.content(), None);

    let green = wheel.slot(&Name::new("Green")).unwrap();
    assert_eq!(green.color(), &SlotColor::Filter(NodePath::from_str("GreenFilter").unwrap()));
    assert_eq!(green.content(), None);

    let blue = wheel.slot(&Name::new("Blue")).unwrap();
    assert_eq!(blue.color(), &SlotColor::Cie(CieColor::new(0.1500, 0.0600, 5.0)));
    assert_eq!(blue.content(), None);
}

#[test]
fn test_gdtf_gobo_wheel_open_and_closed_slots() {
    let gdtf = load_complete_gdtf();
    let wheel = gdtf.wheel(&Name::new("GoboWheel")).unwrap();

    let open = wheel.slot(&Name::new("Open")).unwrap();
    assert_eq!(open.content(), None);
    assert_eq!(open.file(), None);

    let closed = wheel.slot(&Name::new("Closed")).unwrap();
    assert_eq!(closed.content(), None);
    assert_eq!(closed.file(), None);
}

#[test]
fn test_gdtf_gobo_wheel_gobo_slot() {
    let gdtf = load_complete_gdtf();
    let wheel = gdtf.wheel(&Name::new("GoboWheel")).unwrap();

    let gobo = wheel.slot(&Name::new("Gobo1")).unwrap();
    assert_eq!(gobo.file().map(|k| k.relative_path()), Some(Path::new("wheels/gobo1.png")));
    assert_eq!(gobo.content(), None);
}

#[test]
fn test_gdtf_gobo_wheel_prism_slot() {
    let gdtf = load_complete_gdtf();
    let wheel = gdtf.wheel(&Name::new("GoboWheel")).unwrap();

    let prism = wheel.slot(&Name::new("Prism")).unwrap();
    let WheelSlotContent::Prism(facets) = prism.content().unwrap() else {
        panic!("expected Prism content");
    };
    assert_eq!(facets.len(), 3);
}

#[test]
fn test_gdtf_gobo_wheel_animation_slot() {
    let gdtf = load_complete_gdtf();
    let wheel = gdtf.wheel(&Name::new("GoboWheel")).unwrap();

    let anim = wheel.slot(&Name::new("AnimWheel")).unwrap();
    assert_eq!(anim.file().map(|k| k.relative_path()), Some(Path::new("wheels/animwheel.png")));

    let WheelSlotContent::AnimationSystem(sys) = anim.content().unwrap() else {
        panic!("expected AnimationSystem content");
    };
    assert_eq!(sys.p1(), rigger::glam::Vec2::new(-0.5, 0.0));
    assert_eq!(sys.p2(), rigger::glam::Vec2::new(0.0, 0.5));
    assert_eq!(sys.p3(), rigger::glam::Vec2::new(0.5, 0.0));
    assert_eq!(sys.radius(), 0.3);
}

#[test]
fn test_physical_descriptions_emitters() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.emitters().len(), 4);

    let white = gdtf.emitter(&Name::new("WhiteLight")).unwrap();
    assert_eq!(white.name().as_str(), "WhiteLight");
    assert_eq!(white.color(), EmitterColor::Color(CieColor::new(0.3127, 0.3290, 100.0)));
    assert_eq!(white.diode_part(), Some("LED-WW-3000K"));
    assert_eq!(white.measurements().len(), 2);

    let m0 = &white.measurements()[0];
    assert_eq!(m0.physical(), 100.0);
    assert_eq!(m0.luminous_intensity(), 800.0);
    assert_eq!(m0.interpolation_to(), InterpolationTo::Linear);
    assert_eq!(m0.points().len(), 4);
    assert_eq!(m0.points()[0].wave_length(), 400.0);
    assert_eq!(m0.points()[0].energy(), 0.02);
    assert_eq!(m0.points()[3].wave_length(), 700.0);
    assert_eq!(m0.points()[3].energy(), 0.05);

    let m1 = &white.measurements()[1];
    assert_eq!(m1.physical(), 50.0);
    assert_eq!(m1.luminous_intensity(), 380.0);
    assert_eq!(m1.interpolation_to(), InterpolationTo::Step);
    assert_eq!(m1.points().len(), 0);

    let red = gdtf.emitter(&Name::new("RedLED")).unwrap();
    assert_eq!(red.color(), EmitterColor::DominantWaveLength(630.0));
    assert_eq!(red.diode_part(), Some("LED-R-630nm"));
    assert_eq!(red.measurements()[0].interpolation_to(), InterpolationTo::Linear);
    assert_eq!(red.measurements()[0].points().len(), 3);

    let green = gdtf.emitter(&Name::new("GreenLED")).unwrap();
    assert_eq!(green.color(), EmitterColor::DominantWaveLength(525.0));
    assert_eq!(green.diode_part(), None);
    assert_eq!(green.measurements()[0].interpolation_to(), InterpolationTo::Log);

    let blue = gdtf.emitter(&Name::new("BlueLED")).unwrap();
    assert_eq!(blue.color(), EmitterColor::DominantWaveLength(465.0));
}

#[test]
fn test_physical_descriptions_filters() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.filters().len(), 2);

    let red = gdtf.filter(&Name::new("RedFilter")).unwrap();
    assert_eq!(red.name().as_str(), "RedFilter");
    assert_color(red.color(), CieColor::new(0.7347, 0.2653, 25.0));
    assert_eq!(red.measurements().len(), 2);

    let m0 = &red.measurements()[0];
    assert_eq!(m0.physical(), 100.0);
    assert_eq!(m0.transmission(), 0.25);
    assert_eq!(m0.interpolation_to(), InterpolationTo::Linear);
    assert_eq!(m0.points().len(), 2);
    assert_eq!(m0.points()[0].wave_length(), 620.0);
    assert_eq!(m0.points()[0].energy(), 0.80);

    let m1 = &red.measurements()[1];
    assert_eq!(m1.physical(), 50.0);
    assert_eq!(m1.transmission(), 0.12);
    assert_eq!(m1.points().len(), 0);

    let green = gdtf.filter(&Name::new("GreenFilter")).unwrap();
    assert_color(green.color(), CieColor::new(0.1596, 0.8404, 70.0));
    assert_eq!(green.measurements()[0].transmission(), 0.70);
}

#[test]
fn test_physical_descriptions_color_spaces() {
    let gdtf = load_complete_gdtf();

    let default_cs = gdtf.color_space().unwrap();
    assert_eq!(default_cs.name().unwrap().as_str(), "Default");
    assert_eq!(default_cs.mode(), &ColorSpaceMode::SRgb);

    assert_eq!(gdtf.additional_color_spaces().len(), 2);

    let custom = gdtf.additional_color_space(&Name::new("CustomSpace")).unwrap();
    assert_eq!(custom.name().unwrap().as_str(), "CustomSpace");
    let ColorSpaceMode::Custom { red, green, blue, white_point } = custom.mode() else {
        panic!("expected Custom mode");
    };
    assert_color(*red, CieColor::new(0.7000, 0.3000, 0.2126));
    assert_color(*green, CieColor::new(0.2000, 0.7000, 0.7152));
    assert_color(*blue, CieColor::new(0.1500, 0.0600, 0.0722));
    assert_color(*white_point, CieColor::new(0.3127, 0.3290, 1.0000));

    let pro_photo = gdtf.additional_color_space(&Name::new("ProPhotoSpace")).unwrap();
    assert_eq!(pro_photo.mode(), &ColorSpaceMode::ProPhoto);
}

#[test]
fn test_physical_descriptions_gamuts() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.gamuts().len(), 1);

    let gamut = gdtf.gamut(&Name::new("FullGamut")).unwrap();
    assert_eq!(gamut.name().unwrap().as_str(), "FullGamut");
    assert_eq!(gamut.points().len(), 3);
    assert_color(gamut.points()[0], CieColor::new(0.7347, 0.2653, 0.2126));
    assert_color(gamut.points()[1], CieColor::new(0.1596, 0.8404, 0.7152));
    assert_color(gamut.points()[2], CieColor::new(0.0366, 0.0001, 0.0722));
}

#[test]
fn test_physical_descriptions_dmx_profiles() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.dmx_profiles().len(), 1);

    let profile = gdtf.dmx_profile(&Name::new("DimmerCurve")).unwrap();
    assert_eq!(profile.name().unwrap().as_str(), "DimmerCurve");
    assert_eq!(profile.points().len(), 3);

    assert_eq!(profile.points()[0].dmx_percentage(), 0.0);
    assert_eq!(profile.points()[0].cfc(), [0.0, 0.0, 0.0, 0.0]);

    assert_eq!(profile.points()[1].dmx_percentage(), 50.0);
    assert_eq!(profile.points()[1].cfc(), [0.2, 0.8, 0.001, 0.0]);

    assert_eq!(profile.points()[2].dmx_percentage(), 100.0);
    assert_eq!(profile.points()[2].cfc(), [1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_physical_descriptions_cri_groups() {
    let gdtf = load_complete_gdtf();
    assert_eq!(gdtf.cri_groups().len(), 2);

    let g0 = &gdtf.cri_groups()[0];
    assert_eq!(g0.color_temperature(), 6000.0);
    assert_eq!(g0.cris().len(), 4);
    assert_eq!(g0.cris()[0].ces(), Ces::new(1).unwrap());
    assert_eq!(g0.cris()[0].color_rendering_index(), 95);
    assert_eq!(g0.cris()[1].ces(), Ces::new(2).unwrap());
    assert_eq!(g0.cris()[1].color_rendering_index(), 93);
    assert_eq!(g0.cris()[2].ces(), Ces::new(3).unwrap());
    assert_eq!(g0.cris()[2].color_rendering_index(), 97);
    assert_eq!(g0.cris()[3].ces(), Ces::new(99).unwrap());
    assert_eq!(g0.cris()[3].color_rendering_index(), 90);

    let g1 = &gdtf.cri_groups()[1];
    assert_eq!(g1.color_temperature(), 3200.0);
    assert_eq!(g1.cris().len(), 2);
    assert_eq!(g1.cris()[0].ces(), Ces::new(1).unwrap());
    assert_eq!(g1.cris()[0].color_rendering_index(), 99);
    assert_eq!(g1.cris()[1].ces(), Ces::new(2).unwrap());
    assert_eq!(g1.cris()[1].color_rendering_index(), 98);
}

#[test]
fn test_physical_descriptions_properties() {
    let gdtf = load_complete_gdtf();

    let props = gdtf.properties();
    let temp = props.operating_temperature().unwrap();
    assert_eq!(temp.low(), -10.0);
    assert_eq!(temp.high(), 45.0);
    assert_eq!(props.weight(), Some(15.5));
    assert_eq!(props.leg_height(), Some(0.12));
}
