use std::{
    net::{Ipv4Addr, Ipv6Addr},
    path::Path,
    str::FromStr,
};

use rigger::mvr::{
    GdtfInfo, Layer, Mvr, NodeId, Object,
    layer::{ObjectIdentifier, ObjectKind, ScaleHandling, SourceType, Transmission},
};

fn load_complete_mvr() -> Mvr {
    Mvr::from_folder(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("mvr")
            .join("complete"),
    )
}

fn load_empty_scene_and_user_data_mvr() -> Mvr {
    Mvr::from_folder(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("mvr")
            .join("empty_scene_and_user_data"),
    )
}

fn object_by_uuid<'a>(mvr: &'a Mvr, uuid: &str) -> &'a Object {
    let id: NodeId<Object> = NodeId::from_str(uuid).unwrap();
    mvr.object(id).unwrap_or_else(|| panic!("Object {uuid} not found"))
}

fn layer_by_uuid<'a>(mvr: &'a Mvr, uuid: &str) -> &'a Layer {
    let id: NodeId<Layer> = NodeId::from_str(uuid).unwrap();
    mvr.layer(id).unwrap_or_else(|| panic!("Layer {uuid} not found"))
}

#[test]
fn test_version() {
    let mvr = load_complete_mvr();
    assert_eq!(mvr.version().major(), 1);
    assert_eq!(mvr.version().minor(), 6);
}

#[test]
fn test_provider() {
    let mvr = load_complete_mvr();
    assert_eq!(mvr.provider().name(), "Handwritten");
    assert_eq!(mvr.provider().version(), "42.0");
}

#[test]
fn test_provider_defaults_when_missing() {
    let mvr = load_empty_scene_and_user_data_mvr();
    assert_eq!(mvr.provider().name(), "Handwritten");
}

#[test]
fn test_layer_count_and_names() {
    let mvr = load_complete_mvr();
    let layers = mvr.layers();
    assert_eq!(layers.len(), 3);
    assert_eq!(layers[0].name(), "Empty Layer");
    assert_eq!(layers[1].name(), "Single Simple Object Layer");
    assert_eq!(layers[2].name(), "Complete Object Layer");
}

#[test]
fn test_layer_lookup_by_id() {
    let mvr = load_complete_mvr();
    let layer = layer_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000002");
    assert_eq!(layer.name(), "Single Simple Object Layer");
}

#[test]
fn test_layer_lookup_returns_none_for_unknown_id() {
    let mvr = load_complete_mvr();
    let id: NodeId<Layer> = NodeId::from_str("00000000-0000-0000-0000-000000000000").unwrap();
    assert!(mvr.layer(id).is_none());
}

#[test]
fn test_empty_layer_has_no_objects() {
    let mvr = load_complete_mvr();
    let layer = layer_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000001");
    assert_eq!(layer.objects().len(), 0);
}

#[test]
fn test_layer_without_matrix_has_identity_transform() {
    let mvr = load_complete_mvr();
    let layer = layer_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000001");
    assert_eq!(*layer.local_transform(), glam::Affine3A::IDENTITY);
}

#[test]
fn test_classes() {
    let mvr = load_complete_mvr();
    let mut classes: Vec<_> = mvr.classes().collect();
    classes.sort_by_key(|c| c.name());
    assert_eq!(classes.len(), 2);
    assert_eq!(classes[0].name(), "Class 1");
    assert_eq!(classes[1].name(), "Class 2");

    let id = NodeId::from_str("deadbeef-0000-0000-0000-000000000064").unwrap();
    let class = mvr.class(id).expect("Expected Class 1 by id");
    assert_eq!(class.name(), "Class 1");
    assert_eq!(class.id(), id);
}

#[test]
fn test_positions() {
    let mvr = load_complete_mvr();
    let mut positions: Vec<_> = mvr.positions().collect();
    positions.sort_by_key(|p| p.name());
    assert_eq!(positions.len(), 2);
    assert_eq!(positions[0].name(), "Position 1");
    assert_eq!(positions[1].name(), "Position 2");

    let id = NodeId::from_str("deadbeef-0000-0000-0000-000000000071").unwrap();
    let pos = mvr.position(id).expect("Expected Position 1 by id");
    assert_eq!(pos.name(), "Position 1");
}

#[test]
fn test_symdefs() {
    let mvr = load_complete_mvr();
    assert_eq!(mvr.symdefs().count(), 5);

    let id = NodeId::from_str("deadbeef-0000-0000-0000-000000000066").unwrap();
    let symdef = mvr.symdef(id).expect("Expected Symdef 'Symbol 1' by id");
    assert_eq!(symdef.name(), "Symbol 1");
    assert_eq!(symdef.id(), id);
}

#[test]
fn test_mapping_definitions() {
    let mvr = load_complete_mvr();
    assert_eq!(mvr.mapping_definitions().count(), 7);

    let id = NodeId::from_str("deadbeef-0000-0000-0000-000000000073").unwrap();
    let md = mvr.mapping_definition(id).expect("Expected MappingDefinition 'Mapping 1'");
    assert_eq!(md.name(), "Mapping 1");
    assert_eq!(md.size_x(), 1920);
    assert_eq!(md.size_y(), 1080);
    assert_eq!(md.source().type_(), SourceType::File);
    assert_eq!(md.source().value(), "movie.mov");
    assert_eq!(md.scale_handling(), ScaleHandling::ScaleKeepRatio);
}

#[test]
fn test_mapping_definition_scale_handling_variants() {
    let mvr = load_complete_mvr();

    let id_ignore = NodeId::from_str("deadbeef-0000-0000-0000-000000000088").unwrap();
    let md_ignore = mvr.mapping_definition(id_ignore).unwrap();
    assert_eq!(md_ignore.scale_handling(), ScaleHandling::ScaleIgnoreRatio);

    let id_center = NodeId::from_str("deadbeef-0000-0000-0000-000000000089").unwrap();
    let md_center = mvr.mapping_definition(id_center).unwrap();
    assert_eq!(md_center.scale_handling(), ScaleHandling::KeepSizeCenter);
}

#[test]
fn test_object_lookup_by_id() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000003");
    assert_eq!(obj.name(), "Simple Object");
}

#[test]
fn test_object_lookup_returns_none_for_unknown_id() {
    let mvr = load_complete_mvr();
    let id: NodeId<Object> = NodeId::from_str("00000000-0000-0000-0000-000000000000").unwrap();
    assert!(mvr.object(id).is_none());
}

#[test]
fn test_object_is_first_in_layer() {
    let mvr = load_complete_mvr();
    let id: NodeId<Object> = NodeId::from_str("deadbeef-0000-0000-0000-000000000003").unwrap();
    let layer = layer_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000002");
    assert_eq!(layer.objects().first().map(Object::id), Some(id));
}

#[test]
fn test_child_object_is_reachable() {
    let mvr = load_complete_mvr();
    let child_id: NodeId<Object> =
        NodeId::from_str("deadbeef-0000-0000-0000-000000000006").unwrap();
    assert!(mvr.object(child_id).is_some());
}

#[test]
fn test_root_objects_count() {
    let mvr = load_complete_mvr();
    assert_eq!(mvr.root_objects().count(), 18);
}

#[test]
fn test_objects_walk_visits_all_objects() {
    let mvr = load_complete_mvr();
    assert_eq!(mvr.objects().count(), 27);
}

#[test]
fn test_layer_walk_visits_all_objects_in_layer() {
    let mvr = load_complete_mvr();
    let layer = layer_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000004");
    assert_eq!(layer.walk().count(), 26);
}

#[test]
fn test_layer_walk_is_depth_first_preorder() {
    let mvr = load_complete_mvr();
    let layer = layer_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000004");

    let names: Vec<_> = layer.walk().map(|o| o.name()).take(4).collect();
    assert_eq!(names[0], "Complete SceneObject 1");
    assert_eq!(names[1], "Focus Point (SceneObject Child)");
    assert_eq!(names[2], "Complete Group 1");
    assert_eq!(names[3], "Child Object 1");
}

#[test]
fn test_object_name() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    assert_eq!(obj.name(), "Complete SceneObject 1");
}

#[test]
fn test_object_class_is_resolved() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");

    let class_id = obj.class().expect("Expected class on Complete SceneObject 1");
    let class = mvr.class(class_id).expect("Expected class to be resolvable");
    assert_eq!(class.name(), "Class 1");
}

#[test]
fn test_object_without_class_has_none() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000003");
    assert!(obj.class().is_none());
}

#[test]
fn test_object_local_transform_identity_when_no_matrix() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000003");
    assert_eq!(*obj.local_transform(), glam::Affine3A::IDENTITY);
}

#[test]
fn test_object_local_transform_translation_divided_by_1000() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let t = obj.local_transform().translation;
    assert!((t.x - 1.0).abs() < 1e-5);
    assert!(t.y.abs() < 1e-5);
    assert!(t.z.abs() < 1e-5);
}

#[test]
fn test_object_world_transform() {
    let mvr = load_complete_mvr();
    let id: NodeId<Object> = NodeId::from_str("deadbeef-0000-0000-0000-000000000005").unwrap();
    let world = mvr.object_world_transform(id).expect("Expected world transform");
    let t = world.translation;
    assert!((t.x - 1.0).abs() < 1e-5);
    assert!(t.y.abs() < 1e-5);
}

#[test]
fn test_child_object_world_transform_accumulates_parent() {
    let mvr = load_complete_mvr();
    let id: NodeId<Object> = NodeId::from_str("deadbeef-0000-0000-0000-000000000006").unwrap();
    let world = mvr.object_world_transform(id).expect("Expected world transform for child");
    let t = world.translation;
    assert!((t.x - 2.0).abs() < 1e-5);
    assert!((t.y - 0.5).abs() < 1e-5);
    assert!(t.z.abs() < 1e-5);
}

#[test]
fn test_object_world_transform_returns_none_for_unknown_id() {
    let mvr = load_complete_mvr();
    let id: NodeId<Object> = NodeId::from_str("00000000-0000-0000-0000-000000000000").unwrap();
    assert!(mvr.object_world_transform(id).is_none());
}

#[test]
fn test_object_kind_variants() {
    let mvr = load_complete_mvr();
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005").kind(),
        ObjectKind::SceneObject(_)
    ));
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007").kind(),
        ObjectKind::GroupObject(_)
    ));
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000011").kind(),
        ObjectKind::FocusPoint(_)
    ));
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009").kind(),
        ObjectKind::Fixture(_)
    ));
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000013").kind(),
        ObjectKind::Truss(_)
    ));
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000015").kind(),
        ObjectKind::Support(_)
    ));
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000017").kind(),
        ObjectKind::VideoScreen(_)
    ));
    assert!(matches!(
        object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000018").kind(),
        ObjectKind::Projector(_)
    ));
}

#[test]
fn test_downcast_returns_correct_type() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    assert!(obj.as_fixture_object().is_some());
    assert!(obj.as_scene_object().is_none());
    assert!(obj.as_truss_object().is_none());
}

#[test]
fn test_identifier_single_variant() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let id = obj.identifier().expect("Expected identifier");
    let ObjectIdentifier::Single { fixture_id, fixture_id_numeric, custom_id, custom_id_type } = id
    else {
        panic!("Expected Single identifier");
    };
    assert_eq!(fixture_id.as_deref(), Some("SCENEOBJECT-0005"));
    assert_eq!(*fixture_id_numeric, Some(1005));
    assert_eq!(*custom_id, Some(4005));
    assert_eq!(*custom_id_type, Some(3005));
}

#[test]
fn test_identifier_multipatch_variant() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000030");
    let id = obj.identifier().expect("Expected identifier");
    let ObjectIdentifier::Multipatch(parent_id) = id else {
        panic!("Expected Multipatch identifier");
    };
    let expected: NodeId<Object> =
        NodeId::from_str("deadbeef-0000-0000-0000-000000000029").unwrap();
    assert_eq!(*parent_id, expected);
}

#[test]
fn test_group_and_focus_have_no_identifier() {
    let mvr = load_complete_mvr();
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007").identifier().is_none());
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000011").identifier().is_none());
}

#[test]
fn test_gdtf_info_present() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let info = obj.gdtf_info().expect("Expected GdtfInfo on Complete Fixture 1");
    assert_eq!(info.gdtf_spec(), "Robe Lighting@Robin Spiider.gdtf");
    assert_eq!(info.gdtf_mode(), "Mode 1 - Standard 16 bit");
}

#[test]
fn test_gdtf_info_absent_when_not_set() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000021");
    assert!(obj.gdtf_info().is_none());
}

#[test]
fn test_group_and_focus_have_no_gdtf_info() {
    let mvr = load_complete_mvr();
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007").gdtf_info().is_none());
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000011").gdtf_info().is_none());
}

#[test]
fn test_object_children() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007");
    let children = obj.children().expect("Expected children on Complete Group 1");
    assert_eq!(children.len(), 2);
    assert_eq!(children[0].name(), "Child Object 1");
    assert_eq!(children[1].name(), "Child Object 2");
}

#[test]
fn test_has_children() {
    let mvr = load_complete_mvr();
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007").has_children());
    assert!(!object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000022").has_children());
}

#[test]
fn test_focus_point_has_no_children() {
    let mvr = load_complete_mvr();
    let fp = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000011");
    assert!(fp.children().is_none());
}

#[test]
fn test_dmx_address_absolute_format() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let addrs = obj.dmx_addresses().expect("Expected DMX addresses");
    assert_eq!(addrs.len(), 1);
    assert_eq!(addrs[0].break_(), 0);
    assert_eq!(addrs[0].absolute_value(), 45);
}

#[test]
fn test_dmx_address_universe_channel_format() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let addrs = obj.dmx_addresses().expect("Expected DMX addresses");
    assert_eq!(addrs.len(), 2);
    assert_eq!(addrs[0].break_(), 0);
    assert_eq!(addrs[0].absolute_value(), 42);
    assert_eq!(addrs[1].break_(), 1);
    assert_eq!(addrs[1].absolute_value(), 21034);
}

#[test]
fn test_dmx_addresses_empty_when_no_addresses() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000003");
    let addrs = obj.dmx_addresses().expect("Expected DMX addresses slice (possibly empty)");
    assert_eq!(addrs.len(), 0);
}

#[test]
fn test_group_and_focus_dmx_addresses_none() {
    let mvr = load_complete_mvr();
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007").dmx_addresses().is_none());
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000011").dmx_addresses().is_none());
}

#[test]
fn test_network_address_ipv4() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let nets = obj.network_addresses().expect("Expected network addresses");
    assert_eq!(nets.len(), 1);
    assert_eq!(nets[0].ipv4(), Some(Ipv4Addr::new(192, 168, 1, 100)));
    assert_eq!(nets[0].subnetmask(), Some(Ipv4Addr::new(255, 255, 255, 0)));
    assert_eq!(nets[0].dhcp(), true);
    assert_eq!(nets[0].hostname(), Some("example-device"));
    assert!(nets[0].ipv6().is_none());
}

#[test]
fn test_network_address_ipv6() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let nets = obj.network_addresses().expect("Expected network addresses");
    assert_eq!(nets.len(), 3);
    let ipv6_net = &nets[1];
    assert_eq!(
        ipv6_net.ipv6(),
        Some("2001:0db8:85a3:0000:0000:8a2e:0370:7344".parse::<Ipv6Addr>().unwrap())
    );
    assert!(ipv6_net.ipv4().is_none());
    assert_eq!(ipv6_net.dhcp(), false);
}

#[test]
fn test_network_address_dhcp_only() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let nets = obj.network_addresses().expect("Expected network addresses");
    let dhcp_net = &nets[2];
    assert_eq!(dhcp_net.dhcp(), true);
    assert!(dhcp_net.ipv4().is_none());
    assert!(dhcp_net.ipv6().is_none());
    assert!(dhcp_net.hostname().is_none());
}

#[test]
fn test_alignments() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let aligns = obj.alignments().expect("Expected alignments");
    assert_eq!(aligns.len(), 1);
    assert_eq!(aligns[0].geometry().to_string(), "Beam1");
    assert_eq!(*aligns[0].up(), glam::Vec3A::new(0.0, 0.0, 1.0));
    assert_eq!(*aligns[0].direction(), glam::Vec3A::new(0.0, 0.0, -1.0));
}

#[test]
fn test_custom_commands() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let cmds = obj.custom_commands().expect("Expected custom commands");
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0].command(), "Body_Tilt.Tilt.Tilt 1,f 0.000000");
}

#[test]
fn test_overwrites() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let ows = obj.overwrites().expect("Expected overwrites");
    assert_eq!(ows.len(), 1);
    assert_eq!(ows[0].universal().to_string(), "SomeUniversalNode");
    assert_eq!(ows[0].target().map(|n| n.to_string()).as_deref(), Some("SomeTargetNode"));
}

#[test]
fn test_connections() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let conns = obj.connections().expect("Expected connections");
    assert_eq!(conns.len(), 1);
    assert_eq!(conns[0].own().to_string(), "Base");
    assert_eq!(conns[0].other().to_string(), "Base.Yoke");

    let expected_target: NodeId<Object> =
        NodeId::from_str("deadbeef-0000-0000-0000-000000000009").unwrap();
    assert_eq!(conns[0].to_object(), expected_target);
}

#[test]
fn test_group_and_focus_return_none_for_shared_methods() {
    let mvr = load_complete_mvr();
    let group = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007");
    assert!(group.alignments().is_none());
    assert!(group.custom_commands().is_none());
    assert!(group.overwrites().is_none());
    assert!(group.connections().is_none());
    assert!(group.network_addresses().is_none());

    let fp = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000011");
    assert!(fp.alignments().is_none());
    assert!(fp.custom_commands().is_none());
}

#[test]
fn test_geometries_present_on_scene_object() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let geos = obj.geometries().expect("Expected geometries");
    assert_eq!(geos.len(), 1);
    assert_eq!(geos[0].model().as_str(), "model1.glb");
}

#[test]
fn test_geometry_local_transform_from_geometry3d_matrix() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000005");
    let geos = obj.geometries().unwrap();
    let t = geos[0].local_transform().translation;
    assert!((t.x - 0.1).abs() < 1e-5);
    assert!(t.y.abs() < 1e-5);
}

#[test]
fn test_geometry_from_symbol_with_matrix_composes_transforms() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000008");
    let geos = obj.geometries().unwrap();
    assert_eq!(geos.len(), 1);
    let t = geos[0].local_transform().translation;
    assert!((t.x - 0.3).abs() < 1e-5);
    assert!(t.y.abs() < 1e-5);
}

#[test]
fn test_object_geometries_world() {
    let mvr = load_complete_mvr();
    let id: NodeId<Object> = NodeId::from_str("deadbeef-0000-0000-0000-000000000005").unwrap();
    let mut geos = mvr.object_geometries_world(id).expect("Expected geometries world");
    let (geo, world) = geos.next().expect("Expected at least one geometry");
    assert_eq!(geo.model().as_str(), "model1.glb");
    let t = world.translation;
    assert!((t.x - 1.1).abs() < 1e-5);
    assert!(t.y.abs() < 1e-5);
}

#[test]
fn test_group_and_fixture_have_no_geometries() {
    let mvr = load_complete_mvr();
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000007").geometries().is_none());
    assert!(object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009").geometries().is_none());
}

#[test]
fn test_fixture_cast_shadow_and_dmx_invert() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    assert_eq!(f.cast_shadow(), true);
    assert_eq!(f.dmx_invert_pan(), true);
    assert_eq!(f.dmx_invert_tilt(), false);
}

#[test]
fn test_fixture_function_and_color() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    assert_eq!(f.function(), Some("This fixture is meant for testing"));
    let color = f.color().expect("Expected color");
    assert!((color.x - 0.314303).abs() < 1e-5);
    assert!((color.y - 0.328065).abs() < 1e-5);
    assert!((color.yy - 87.699166).abs() < 1e-3);
}

#[test]
fn test_fixture_focus_point_reference() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    let focus_id = f.focus_point().expect("Expected focus point reference");
    let expected: NodeId<Object> =
        NodeId::from_str("deadbeef-0000-0000-0000-000000000006").unwrap();
    assert_eq!(focus_id, expected);
}

#[test]
fn test_fixture_position_reference() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    let pos_id = f.position().expect("Expected position reference");
    let pos = mvr.position(pos_id).expect("Expected position to resolve");
    assert_eq!(pos.name(), "Position 1");
}

#[test]
fn test_fixture_gobo() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    let gobo = f.gobo().expect("Expected gobo");
    assert_eq!(gobo.resource().as_str(), "gobo.png");
    assert!((gobo.rotation() - 32.5).abs() < 1e-5);
}

#[test]
fn test_fixture_mappings() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    let mappings = f.mappings();
    assert_eq!(mappings.len(), 1);
    let expected_def: NodeId<_> = NodeId::from_str("deadbeef-0000-0000-0000-000000000073").unwrap();
    assert_eq!(mappings[0].linked_def(), expected_def);
    assert_eq!(mappings[0].ux(), 10);
    assert_eq!(mappings[0].uy(), 10);
    assert_eq!(mappings[0].ox(), 5);
    assert_eq!(mappings[0].oy(), 5);
    assert!((mappings[0].rz() - 45.0).abs() < 1e-5);
}

#[test]
fn test_fixture_protocols() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    let protos = f.protocols();
    assert_eq!(protos.len(), 6);
    assert_eq!(protos[0].geometry().to_string(), "NetworkInOut_1");
    assert_eq!(protos[0].type_(), Some("Art-Net"));
    assert_eq!(protos[0].transmission(), None);
    assert_eq!(protos[1].name(), "NDI 1");
    assert_eq!(protos[2].transmission(), Some(Transmission::Unicast));
    assert_eq!(protos[3].transmission(), Some(Transmission::Multicast));
    assert_eq!(protos[4].transmission(), Some(Transmission::Broadcast));
    assert_eq!(protos[5].transmission(), Some(Transmission::Anycast));
}

#[test]
fn test_fixture_unit_number() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    assert_eq!(f.unit_number(), Some(2009));
}

#[test]
fn test_fixture_child_position() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000009");
    let f = obj.as_fixture_object().unwrap();
    assert_eq!(f.child_position().map(|n| n.to_string()).as_deref(), Some("Base.Yoke"));
}

#[test]
fn test_truss_fields() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000013");
    let t = obj.as_truss_object().unwrap();
    assert_eq!(t.function(), Some("This truss is meant for testing"));
    assert_eq!(t.child_position().map(|n| n.to_string()).as_deref(), Some("Base.Yoke"));
    assert_eq!(t.cast_shadow(), true);
    assert_eq!(t.unit_number(), Some(2013));

    let pos_id = t.position().expect("Expected position reference on truss");
    let pos = mvr.position(pos_id).expect("Expected position to resolve");
    assert_eq!(pos.name(), "Position 1");

    assert_eq!(t.children().len(), 1);
}

#[test]
fn test_support_fields() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000015");
    let s = obj.as_support_object().unwrap();
    assert_eq!(s.function(), Some("This support is meant for testing"));
    assert!((s.chain_length() - 2.5).abs() < 1e-5);
    assert_eq!(s.cast_shadow(), true);
    assert_eq!(s.unit_number(), Some(2015));
    assert_eq!(s.children().len(), 1);
}

#[test]
fn test_video_screen_sources() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000017");
    let vs = obj.as_video_screen_object().unwrap();
    assert_eq!(vs.function(), Some("This video screen is meant for testing"));
    let sources = vs.sources();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].type_(), SourceType::File);
    assert_eq!(sources[0].value(), "movie.mov");
    assert_eq!(sources[0].linked_geometry().to_string(), "Display1");
}

#[test]
fn test_projector_projection_and_scale_handling() {
    let mvr = load_complete_mvr();
    let obj = object_by_uuid(&mvr, "deadbeef-0000-0000-0000-000000000018");
    let p = obj.as_projector_object().unwrap();
    let projs = p.projections();
    assert_eq!(projs.len(), 1);
    assert_eq!(projs[0].source().type_(), SourceType::File);
    assert_eq!(projs[0].source().value(), "projector_content.mov");
    assert_eq!(projs[0].scale_handling(), ScaleHandling::ScaleKeepRatio);
}

#[test]
fn test_gdtf_loading() {
    let mvr = load_complete_mvr();

    assert_eq!(mvr.gdtfs().count(), 1);

    let gdtf = mvr
        .gdtf(&GdtfInfo::new("Robe Lighting@Robin Spiider.gdtf", "Mode 10 - Pattern full RGBW"))
        .expect("Should have GDTF file");
    assert_eq!(gdtf.bundle().description().data_version, "1.2");
}
