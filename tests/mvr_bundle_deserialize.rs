use std::path::Path;

use rigger::mvr::{
    Mvr,
    bundle::{ChildListContent, ResourceKey, ResourceKind, Scale, SourceType, Transmission},
};

fn load_complete_mvr() -> Mvr {
    Mvr::from_folder(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("complete_mvr"),
    )
}

fn load_empty_scene_data_mvr() -> Mvr {
    Mvr::from_folder(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("empty_scene_data"),
    )
}

fn load_empty_scene_and_user_data_mvr() -> Mvr {
    Mvr::from_folder(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("empty_scene_and_user_data"),
    )
}

#[test]
fn test_mvr_bundle_description() {
    let mvr = load_complete_mvr();

    let desc = mvr.bundle().description();

    assert_eq!(desc.provider, Some("Handwritten".to_string()));
    assert_eq!(desc.provider_version, Some("42.0".to_string()));
    assert_eq!(desc.ver_major, 1);
    assert_eq!(desc.ver_minor, 6);
}

#[test]
fn test_mvr_bundle_user_data() {
    let mvr = load_complete_mvr();

    let desc = mvr.bundle().description();

    let user_data = desc.user_data.as_ref().expect("Expected UserData to be present");
    assert_eq!(user_data.data.len(), 2);

    assert_eq!(user_data.data[0].provider, "Vectorworks");
    assert_eq!(user_data.data[0].ver, "0.1");

    assert_eq!(user_data.data[1].provider, "VectorworksLitFiles");
    assert_eq!(user_data.data[1].ver, "0.1");
}

#[test]
fn test_mvr_bundle_empty_scene_data() {
    let mvr = load_empty_scene_data_mvr();

    let desc = mvr.bundle().description();

    assert_eq!(desc.provider, Some("Handwritten".to_string()));
    assert_eq!(desc.provider_version, Some("42.0".to_string()));
    assert_eq!(desc.ver_major, 1);
    assert_eq!(desc.ver_minor, 6);

    let user_data = desc.user_data.as_ref().expect("Expected UserData to be present");
    assert_eq!(user_data.data.len(), 2);
    assert_eq!(user_data.data[0].provider, "Vectorworks");
    assert_eq!(user_data.data[0].ver, "0.1");
    assert_eq!(user_data.data[1].provider, "VectorworksLitFiles");
    assert_eq!(user_data.data[1].ver, "0.1");

    assert!(desc.scene.aux_data.is_some());
    let aux = desc.scene.aux_data.as_ref().expect("Expected AUXData to be present");
    assert_eq!(aux.class.len(), 0);
    assert_eq!(aux.symdef.len(), 0);
    assert_eq!(aux.position.len(), 0);
    assert_eq!(aux.mapping_definition.len(), 0);

    assert_eq!(desc.scene.layers.layer.len(), 0);
}

#[test]
fn test_mvr_bundle_empty_scene_and_user_data() {
    let mvr = load_empty_scene_and_user_data_mvr();

    let desc = mvr.bundle().description();

    assert_eq!(desc.provider, Some("Handwritten".to_string()));
    assert_eq!(desc.provider_version, Some("42.0".to_string()));
    assert_eq!(desc.ver_major, 1);
    assert_eq!(desc.ver_minor, 6);

    let user_data = desc.user_data.as_ref().expect("Expected UserData to be present");
    assert_eq!(user_data.data.len(), 0);

    assert!(desc.scene.aux_data.is_none());
    assert_eq!(desc.scene.layers.layer.len(), 0);
}

#[test]
fn test_mvr_bundle_aux_data() {
    let mvr = load_complete_mvr();

    let aux =
        mvr.bundle().description().scene.aux_data.as_ref().expect("Expected AUXData to be present");

    assert_eq!(aux.class.len(), 2);
    assert_eq!(aux.symdef.len(), 5);
    assert_eq!(aux.position.len(), 2);
    assert_eq!(aux.mapping_definition.len(), 7);

    assert_eq!(aux.class[0].uuid, "deadbeef-0000-0000-0000-000000000064");
    assert_eq!(aux.class[0].name, "Class 1");
    assert_eq!(aux.class[1].uuid, "deadbeef-0000-0000-0000-000000000063");
    assert_eq!(aux.class[1].name, "Class 2");

    let symdef_1 = aux
        .symdef
        .iter()
        .find(|s| s.uuid == "deadbeef-0000-0000-0000-000000000066")
        .expect("Expected Symdef 'Symbol 1'");
    assert_eq!(symdef_1.name, "Symbol 1");
    assert_eq!(symdef_1.child_list.geometry_3d.len(), 1);
    assert_eq!(symdef_1.child_list.geometry_3d[0].file_name, "model1.glb");
    assert_eq!(
        symdef_1.child_list.geometry_3d[0].matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{100.0,0.0,0.0}")
    );
    assert_eq!(symdef_1.child_list.symbol.len(), 0);

    let symdef_2 = aux
        .symdef
        .iter()
        .find(|s| s.uuid == "deadbeef-0000-0000-0000-000000000080")
        .expect("Expected Symdef 'Symbol 2'");
    assert_eq!(symdef_2.name, "Symbol 2");
    assert_eq!(symdef_2.child_list.geometry_3d.len(), 1);
    assert_eq!(symdef_2.child_list.geometry_3d[0].file_name, "model2.glb");
    assert_eq!(symdef_2.child_list.geometry_3d[0].matrix, None);
    assert_eq!(symdef_2.child_list.symbol.len(), 0);

    let symdef_via_symbol = aux
        .symdef
        .iter()
        .find(|s| s.uuid == "deadbeef-0000-0000-0000-000000000081")
        .expect("Expected Symdef 'Symdef via Symbol'");
    assert_eq!(symdef_via_symbol.name, "Symdef via Symbol");
    assert_eq!(symdef_via_symbol.child_list.geometry_3d.len(), 0);
    assert_eq!(symdef_via_symbol.child_list.symbol.len(), 1);
    assert_eq!(symdef_via_symbol.child_list.symbol[0].uuid, "deadbeef-0000-0000-0000-000000000082");
    assert_eq!(
        symdef_via_symbol.child_list.symbol[0].symdef,
        "deadbeef-0000-0000-0000-000000000066"
    );
    assert!(symdef_via_symbol.child_list.symbol[0].matrix.is_none());

    let symdef_empty_child_list = aux
        .symdef
        .iter()
        .find(|s| s.uuid == "deadbeef-0000-0000-0000-000000000083")
        .expect("Expected Symdef with empty ChildList");
    assert_eq!(symdef_empty_child_list.name, "Empty ChildList");
    assert_eq!(symdef_empty_child_list.child_list.geometry_3d.len(), 0);
    assert_eq!(symdef_empty_child_list.child_list.symbol.len(), 0);

    let symdef_symbol_and_geometry = aux
        .symdef
        .iter()
        .find(|s| s.uuid == "deadbeef-0000-0000-0000-000000000084")
        .expect("Expected Symdef with Symbol and Geometry3D");
    assert_eq!(symdef_symbol_and_geometry.name, "Symbol and Geometry3D");
    assert_eq!(symdef_symbol_and_geometry.child_list.geometry_3d.len(), 1);
    assert_eq!(symdef_symbol_and_geometry.child_list.geometry_3d[0].file_name, "model3.glb");
    assert_eq!(symdef_symbol_and_geometry.child_list.symbol.len(), 1);
    assert_eq!(
        symdef_symbol_and_geometry.child_list.symbol[0].uuid,
        "deadbeef-0000-0000-0000-000000000085"
    );
    assert_eq!(
        symdef_symbol_and_geometry.child_list.symbol[0].symdef,
        "deadbeef-0000-0000-0000-000000000080"
    );

    assert_eq!(aux.position[0].uuid, "deadbeef-0000-0000-0000-000000000071");
    assert_eq!(aux.position[0].name, "Position 1");
    assert_eq!(aux.position[1].uuid, "deadbeef-0000-0000-0000-000000000086");
    assert_eq!(aux.position[1].name, "Position 2");

    let mapping_1 = aux
        .mapping_definition
        .iter()
        .find(|m| m.uuid == "deadbeef-0000-0000-0000-000000000073")
        .expect("Expected MappingDefinition 'Mapping 1'");
    assert_eq!(mapping_1.name, "Mapping 1");
    assert_eq!(mapping_1.size_x, 1920);
    assert_eq!(mapping_1.size_y, 1080);
    assert_eq!(mapping_1.source.linked_geometry, "Display1");
    assert_eq!(mapping_1.source.r#type, SourceType::File);
    assert_eq!(mapping_1.source.content, "movie.mov");
    assert!(mapping_1.scale_handeling.is_none());

    let mapping_2 = aux
        .mapping_definition
        .iter()
        .find(|m| m.uuid == "deadbeef-0000-0000-0000-000000000087")
        .expect("Expected MappingDefinition 'Mapping 2'");
    assert_eq!(mapping_2.name, "Mapping 2");
    assert_eq!(mapping_2.size_x, 1280);
    assert_eq!(mapping_2.size_y, 720);
    assert_eq!(mapping_2.source.linked_geometry, "Display2");
    assert_eq!(mapping_2.source.r#type, SourceType::File);
    assert_eq!(mapping_2.source.content, "image.png");
    let sh = mapping_2.scale_handeling.as_ref().expect("Expected ScaleHandeling to be present");
    assert_eq!(sh.r#enum, Scale::ScaleKeepRatio);

    let mapping_3 = aux
        .mapping_definition
        .iter()
        .find(|m| m.uuid == "deadbeef-0000-0000-0000-000000000088")
        .expect("Expected MappingDefinition 'Mapping 3'");
    let sh = mapping_3.scale_handeling.as_ref().expect("Expected ScaleHandeling to be present");
    assert_eq!(sh.r#enum, Scale::ScaleIgnoreRatio);

    let mapping_4 = aux
        .mapping_definition
        .iter()
        .find(|m| m.uuid == "deadbeef-0000-0000-0000-000000000089")
        .expect("Expected MappingDefinition 'Mapping 4'");
    let sh = mapping_4.scale_handeling.as_ref().expect("Expected ScaleHandeling to be present");
    assert_eq!(sh.r#enum, Scale::KeepSizeCenter);

    let mapping_5 = aux
        .mapping_definition
        .iter()
        .find(|m| m.uuid == "deadbeef-0000-0000-0000-000000000090")
        .expect("Expected MappingDefinition 'Mapping 5'");
    assert_eq!(mapping_5.source.r#type, SourceType::Ndi);

    let mapping_6 = aux
        .mapping_definition
        .iter()
        .find(|m| m.uuid == "deadbeef-0000-0000-0000-000000000091")
        .expect("Expected MappingDefinition 'Mapping 6'");
    assert_eq!(mapping_6.source.r#type, SourceType::Citp);

    let mapping_7 = aux
        .mapping_definition
        .iter()
        .find(|m| m.uuid == "deadbeef-0000-0000-0000-000000000092")
        .expect("Expected MappingDefinition 'Mapping 7'");
    assert_eq!(mapping_7.source.r#type, SourceType::CaptureDevice);
}

#[test]
fn test_mvr_bundle_layers_empty_and_simple_layers_parse() {
    let mvr = load_complete_mvr();
    let desc = mvr.bundle().description();

    let empty = desc
        .scene
        .layers
        .layer
        .iter()
        .find(|l| l.name == "Empty Layer")
        .expect("Expected 'Empty Layer'");
    assert_eq!(empty.uuid, "deadbeef-0000-0000-0000-000000000001");
    assert!(empty.child_list.is_none());

    let simple = desc
        .scene
        .layers
        .layer
        .iter()
        .find(|l| l.name == "Single Simple Object Layer")
        .expect("Expected 'Single Simple Object Layer'");
    assert_eq!(simple.uuid, "deadbeef-0000-0000-0000-000000000002");

    let simple_cl = simple.child_list.as_ref().expect("Expected ChildList in simple layer");
    assert_eq!(simple_cl.content.len(), 1);

    let ChildListContent::SceneObject(o) = &simple_cl.content[0] else {
        panic!("Expected SceneObject in simple layer");
    };

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000003");
    assert_eq!(o.name, "Simple Object");
    assert_eq!(o.multipatch, "");
    assert!(o.matrix.is_none());
    assert!(o.classing.is_none());
    assert!(o.gdtf_spec.is_none());
    assert!(o.gdtf_mode.is_none());
    assert!(o.cast_shadow.is_none());
    assert!(o.addresses.is_none());
    assert!(o.alignments.is_none());
    assert!(o.custom_commands.is_none());
    assert!(o.overwrites.is_none());
    assert!(o.connections.is_none());
    assert!(o.fixture_id.is_none());
    assert!(o.fixture_id_numeric.is_none());
    assert!(o.fixture_type_id.is_none());
    assert!(o.unit_number.is_none());
    assert!(o.custom_id_type.is_none());
    assert!(o.custom_id.is_none());
    assert!(o.child_list.is_none());
}

#[test]
fn test_mvr_bundle_minimal_objects_parse_defaults() {
    let mvr = load_complete_mvr();
    let desc = mvr.bundle().description();

    let layer = desc
        .scene
        .layers
        .layer
        .iter()
        .find(|l| l.name == "Complete Object Layer")
        .expect("Expected 'Complete Object Layer'");
    let cl = layer.child_list.as_ref().expect("Expected ChildList in complete layer");

    let minimal_scene_object = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::SceneObject(o) if o.name == "Minimal SceneObject 1" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal SceneObject 1");
    assert_eq!(minimal_scene_object.uuid, "deadbeef-0000-0000-0000-000000000021");
    assert_eq!(minimal_scene_object.fixture_id.as_deref(), Some("SCENEOBJECT-0021"));
    assert_eq!(minimal_scene_object.fixture_id_numeric, Some(1021));

    let minimal_group_object = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::GroupObject(o) if o.name == "Minimal Group 1" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal Group 1");
    assert_eq!(minimal_group_object.uuid, "deadbeef-0000-0000-0000-000000000022");
    assert!(minimal_group_object.matrix.is_none());
    assert!(minimal_group_object.classing.is_none());
    assert_eq!(minimal_group_object.child_list.content.len(), 0);

    let minimal_focus_point = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::FocusPoint(o) if o.name == "Minimal Point 2" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal Point 2");
    assert_eq!(minimal_focus_point.uuid, "deadbeef-0000-0000-0000-000000000023");
    assert!(minimal_focus_point.matrix.is_none());
    assert!(minimal_focus_point.classing.is_none());

    let minimal_fixture = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::Fixture(o) if o.name == "Minimal Fixture 1" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal Fixture 1");
    assert_eq!(minimal_fixture.uuid, "deadbeef-0000-0000-0000-000000000024");
    assert_eq!(minimal_fixture.multipatch, "");
    assert!(minimal_fixture.matrix.is_none());
    assert!(minimal_fixture.classing.is_none());
    assert!(minimal_fixture.gdtf_spec.is_none());
    assert!(minimal_fixture.gdtf_mode.is_none());
    assert!(minimal_fixture.focus.is_none());
    assert!(minimal_fixture.cast_shadow.is_none());
    assert!(minimal_fixture.dmx_invert_pan.is_none());
    assert!(minimal_fixture.dmx_invert_tilt.is_none());
    assert!(minimal_fixture.position.is_none());
    assert!(minimal_fixture.function.is_none());
    assert_eq!(minimal_fixture.fixture_id, "FIXTURE-0024");
    assert_eq!(minimal_fixture.fixture_id_numeric, Some(1024));
    assert_eq!(minimal_fixture.unit_number, 2024);
    assert!(minimal_fixture.child_position.is_none());
    assert!(minimal_fixture.addresses.is_none());
    assert!(minimal_fixture.protocols.is_none());
    assert!(minimal_fixture.alignments.is_none());
    assert!(minimal_fixture.custom_commands.is_none());
    assert!(minimal_fixture.overwrites.is_none());
    assert!(minimal_fixture.connections.is_none());
    assert!(minimal_fixture.color.is_none());
    assert!(minimal_fixture.custom_id_type.is_none());
    assert!(minimal_fixture.custom_id.is_none());
    assert!(minimal_fixture.mappings.is_none());
    assert!(minimal_fixture.gobo.is_none());
    assert!(minimal_fixture.child_list.is_none());

    let minimal_truss = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::Truss(o) if o.name == "Minimal Truss 1" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal Truss 1");
    assert_eq!(minimal_truss.uuid, "deadbeef-0000-0000-0000-000000000025");
    assert_eq!(minimal_truss.multipatch, "");
    assert!(minimal_truss.matrix.is_none());
    assert!(minimal_truss.classing.is_none());
    assert!(minimal_truss.position.is_none());
    assert_eq!(minimal_truss.fixture_id, "TRUSS-0025");
    assert_eq!(minimal_truss.fixture_id_numeric, Some(1025));

    let minimal_support = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::Support(o) if o.name == "Minimal Support 1" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal Support 1");
    assert_eq!(minimal_support.uuid, "deadbeef-0000-0000-0000-000000000026");
    assert_eq!(minimal_support.multipatch, "");
    assert!(minimal_support.matrix.is_none());
    assert!(minimal_support.classing.is_none());
    assert!(minimal_support.position.is_none());
    assert_eq!(minimal_support.fixture_id, "SUPPORT-0026");
    assert_eq!(minimal_support.fixture_id_numeric, Some(1026));

    let minimal_video_screen = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::VideoScreen(o) if o.name == "Minimal VideoScreen 1" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal VideoScreen 1");
    assert_eq!(minimal_video_screen.uuid, "deadbeef-0000-0000-0000-000000000027");
    assert_eq!(minimal_video_screen.multipatch, "");
    assert!(minimal_video_screen.matrix.is_none());
    assert!(minimal_video_screen.classing.is_none());
    assert!(minimal_video_screen.sources.is_none());
    assert_eq!(minimal_video_screen.fixture_id, "SCREEN-0027");
    assert_eq!(minimal_video_screen.fixture_id_numeric, Some(1027));

    let minimal_projector = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::Projector(o) if o.name == "Minimal Projector 1" => Some(o),
            _ => None,
        })
        .expect("Expected Minimal Projector 1");
    assert_eq!(minimal_projector.uuid, "deadbeef-0000-0000-0000-000000000028");
    assert_eq!(minimal_projector.multipatch, "");
    assert!(minimal_projector.matrix.is_none());
    assert!(minimal_projector.classing.is_none());
    assert_eq!(minimal_projector.fixture_id, "PROJECTOR-0028");
    assert_eq!(minimal_projector.fixture_id_numeric, Some(1028));

    let multipatch_parent = cl
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::SceneObject(o) if o.name == "Parent for Multipatch" => Some(o),
            _ => None,
        })
        .expect("Expected Parent for Multipatch");
    assert_eq!(multipatch_parent.uuid, "deadbeef-0000-0000-0000-000000000029");
    assert_eq!(multipatch_parent.fixture_id.as_deref(), Some("SCENEOBJECT-0029"));
    assert_eq!(multipatch_parent.fixture_id_numeric, Some(1029));
    assert_eq!(multipatch_parent.custom_id_type, Some(3029));
    assert_eq!(multipatch_parent.custom_id, Some(4029));

    let child = multipatch_parent
        .child_list
        .as_ref()
        .expect("Expected multipatch parent ChildList")
        .content
        .iter()
        .find_map(|i| match i {
            ChildListContent::SceneObject(o) => Some(o),
            _ => None,
        })
        .expect("Expected multipatch child SceneObject");
    assert_eq!(child.uuid, "deadbeef-0000-0000-0000-000000000030");
    assert_eq!(child.multipatch, "deadbeef-0000-0000-0000-000000000029");
}

fn find_complete_layer_object<'a, T>(
    mvr: &'a Mvr,
    extract: impl Fn(&'a ChildListContent) -> Option<&'a T>,
    label: &str,
) -> &'a T {
    let desc = mvr.bundle().description();
    let layer = desc
        .scene
        .layers
        .layer
        .iter()
        .find(|l| l.name == "Complete Object Layer")
        .expect("Expected 'Complete Object Layer'");
    layer
        .child_list
        .as_ref()
        .expect("Expected ChildList in complete layer")
        .content
        .iter()
        .find_map(extract)
        .unwrap_or_else(|| panic!("Expected {label}"))
}

#[test]
fn test_complete_scene_object_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::SceneObject(o) if o.name == "Complete SceneObject 1" => Some(o),
            _ => None,
        },
        "Complete SceneObject 1",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000005");
    assert_eq!(o.multipatch, "");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{1000.0,0.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(o.gdtf_spec.as_deref(), Some("Robe Lighting@Robin Spiider.gdtf"));
    assert_eq!(o.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(o.cast_shadow, Some(true));

    let addresses = o.addresses.as_ref().expect("Expected addresses");
    assert_eq!(addresses.address.len(), 2);
    assert_eq!(addresses.address[0].r#break, 0);
    assert_eq!(addresses.address[0].content, "42");
    assert_eq!(addresses.address[1].r#break, 1);
    assert_eq!(addresses.address[1].content, "42.42");
    assert_eq!(addresses.network.len(), 1);
    assert_eq!(addresses.network[0].geometry, "ethernet_1");
    assert_eq!(addresses.network[0].ipv_4.as_deref(), Some("192.168.1.100"));
    assert_eq!(addresses.network[0].subnetmask.as_deref(), Some("255.255.255.0"));
    assert_eq!(addresses.network[0].dhcp.as_deref(), Some("on"));
    assert_eq!(addresses.network[0].hostname.as_deref(), Some("example-device"));
    assert!(addresses.network[0].ipv_6.is_none());

    let alignments = o.alignments.as_ref().expect("Expected alignments");
    assert_eq!(alignments.alignment.len(), 1);
    assert_eq!(alignments.alignment[0].geometry.as_deref(), Some("Beam1"));
    assert_eq!(alignments.alignment[0].up, "0,0,1");
    assert_eq!(alignments.alignment[0].direction, "0,0,-1");

    let custom_commands = o.custom_commands.as_ref().expect("Expected custom_commands");
    assert_eq!(custom_commands.custom_command.len(), 1);
    assert_eq!(custom_commands.custom_command[0], "Body_Tilt.Tilt.Tilt 1,f 0.000000");

    let overwrites = o.overwrites.as_ref().expect("Expected overwrites");
    assert_eq!(overwrites.overwrite.len(), 1);
    assert_eq!(overwrites.overwrite[0].universal, "SomeUniversalNode");
    assert_eq!(overwrites.overwrite[0].target, "SomeTargetNode");

    let connections = o.connections.as_ref().expect("Expected connections");
    assert_eq!(connections.connection.len(), 1);
    assert_eq!(connections.connection[0].own, "Base");
    assert_eq!(connections.connection[0].other, "Base.Yoke");
    assert_eq!(connections.connection[0].to_object, "deadbeef-0000-0000-0000-000000000009");

    assert_eq!(o.fixture_id.as_deref(), Some("SCENEOBJECT-0005"));
    assert_eq!(o.fixture_id_numeric, Some(1005));
    assert_eq!(o.unit_number, Some(2005));
    assert_eq!(o.custom_id_type, Some(3005));
    assert_eq!(o.custom_id, Some(4005));
    assert_eq!(o.fixture_type_id, Some(1));

    let child_list = o.child_list.as_ref().expect("Expected SceneObject child list");
    assert_eq!(child_list.content.len(), 1);
    let ChildListContent::FocusPoint(child_fp) = &child_list.content[0] else {
        panic!("Expected FocusPoint as SceneObject child");
    };
    assert_eq!(child_fp.uuid, "deadbeef-0000-0000-0000-000000000006");
    assert_eq!(
        child_fp.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{1000.0,500.0,0.0}")
    );
    assert_eq!(child_fp.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
}

#[test]
fn test_complete_group_object_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::GroupObject(o) if o.name == "Complete Group 1" => Some(o),
            _ => None,
        },
        "Complete Group 1",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000007");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{2000.0,0.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(o.child_list.content.len(), 2);

    let ChildListContent::SceneObject(child_1) = &o.child_list.content[0] else {
        panic!("Expected SceneObject as first GroupObject child");
    };
    assert_eq!(child_1.uuid, "deadbeef-0000-0000-0000-000000000008");
    assert_eq!(child_1.geometries.symbol.len(), 1);
    assert_eq!(child_1.geometries.symbol[0].uuid, "deadbeef-0000-0000-0000-000000000067");
    assert_eq!(child_1.geometries.symbol[0].symdef, "deadbeef-0000-0000-0000-000000000066");
    assert_eq!(
        child_1.geometries.symbol[0].matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{200.0,0.0,0.0}")
    );

    let ChildListContent::SceneObject(child_2) = &o.child_list.content[1] else {
        panic!("Expected SceneObject as second GroupObject child");
    };
    assert_eq!(child_2.uuid, "deadbeef-0000-0000-0000-000000000010");
    assert_eq!(child_2.geometries.symbol.len(), 1);
    assert_eq!(
        child_2.geometries.symbol[0].matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{200.0,500.0,0.0}")
    );
}

#[test]
fn test_complete_focus_point_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::FocusPoint(o) if o.name == "Complete Point 2" => Some(o),
            _ => None,
        },
        "Complete Point 2",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000011");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{2500.0,500.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
}

#[test]
fn test_complete_fixture_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::Fixture(o) if o.name == "Complete Fixture 1" => Some(o),
            _ => None,
        },
        "Complete Fixture 1",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000009");
    assert_eq!(o.multipatch, "");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{3000.0,0.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(o.gdtf_spec.as_deref(), Some("Robe Lighting@Robin Spiider.gdtf"));
    assert_eq!(o.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(o.focus.as_deref(), Some("deadbeef-0000-0000-0000-000000000006"));
    assert_eq!(o.cast_shadow, Some(true));
    assert_eq!(o.dmx_invert_pan, Some(true));
    assert_eq!(o.dmx_invert_tilt, Some(false));
    assert_eq!(o.position.as_deref(), Some("deadbeef-0000-0000-0000-000000000071"));
    assert_eq!(o.function.as_deref(), Some("This fixture is meant for testing"));
    assert_eq!(o.fixture_id, "FIXTURE-0009");
    assert_eq!(o.fixture_id_numeric, Some(1009));
    assert_eq!(o.unit_number, 2009);
    assert_eq!(o.child_position.as_deref(), Some("Base.Yoke"));
    assert_eq!(o.fixture_type_id, Some(1));
    assert_eq!(o.custom_id_type, Some(3009));
    assert_eq!(o.custom_id, Some(4009));

    let addresses = o.addresses.as_ref().expect("Expected fixture addresses");
    assert_eq!(addresses.address.len(), 1);
    assert_eq!(addresses.address[0].r#break, 0);
    assert_eq!(addresses.address[0].content, "45");
    assert_eq!(addresses.network.len(), 3);
    assert_eq!(addresses.network[0].geometry, "ethernet_1");
    assert_eq!(addresses.network[0].ipv_4.as_deref(), Some("192.168.11.5"));
    assert_eq!(addresses.network[0].subnetmask.as_deref(), Some("255.255.0.0"));
    assert!(addresses.network[0].ipv_6.is_none());
    assert_eq!(addresses.network[1].geometry, "ethernet_2");
    assert_eq!(
        addresses.network[1].ipv_6.as_deref(),
        Some("2001:0db8:85a3:0000:0000:8a2e:0370:7344")
    );
    assert!(addresses.network[1].ipv_4.is_none());
    assert_eq!(addresses.network[2].geometry, "wireless_1");
    assert_eq!(addresses.network[2].dhcp.as_deref(), Some("on"));

    let protocols = o.protocols.as_ref().expect("Expected protocols");
    assert_eq!(protocols.protocol.len(), 6);
    assert_eq!(protocols.protocol[0].geometry, "NetworkInOut_1");
    assert_eq!(protocols.protocol[0].name, "");
    assert_eq!(protocols.protocol[0].r#type, "Art-Net");
    assert_eq!(protocols.protocol[0].transmission, None);
    assert_eq!(protocols.protocol[1].geometry, "NetworkInOut_3");
    assert_eq!(protocols.protocol[1].name, "NDI 1");
    assert_eq!(protocols.protocol[1].r#type, "NDI");
    assert_eq!(protocols.protocol[1].transmission, None);
    assert_eq!(protocols.protocol[2].transmission, Some(Transmission::Unicast));
    assert_eq!(protocols.protocol[3].transmission, Some(Transmission::Multicast));
    assert_eq!(protocols.protocol[4].transmission, Some(Transmission::Broadcast));
    assert_eq!(protocols.protocol[5].transmission, Some(Transmission::Anycast));

    let alignments = o.alignments.as_ref().expect("Expected fixture alignments");
    assert_eq!(alignments.alignment.len(), 1);
    assert_eq!(alignments.alignment[0].geometry.as_deref(), Some("Beam"));
    assert_eq!(alignments.alignment[0].up, "0,0,1");
    assert_eq!(alignments.alignment[0].direction, "0,0,-1");

    let custom_commands = o.custom_commands.as_ref().expect("Expected fixture custom_commands");
    assert_eq!(custom_commands.custom_command.len(), 2);
    assert_eq!(custom_commands.custom_command[0], "Body_Pan,f 50");
    assert_eq!(custom_commands.custom_command[1], "Yoke_Tilt,f 50");

    let overwrites = o.overwrites.as_ref().expect("Expected fixture overwrites");
    assert_eq!(overwrites.overwrite.len(), 4);
    assert_eq!(overwrites.overwrite[0].universal, "Universal Wheel 1.Universal Wheel Slot 1");
    assert_eq!(overwrites.overwrite[0].target, "Wheel 1.Wheel Slot");
    assert_eq!(overwrites.overwrite[1].universal, "Universal Emitter 1");
    assert_eq!(overwrites.overwrite[1].target, "Emitter 1");
    assert_eq!(overwrites.overwrite[2].universal, "Universal Filter 1");
    assert_eq!(overwrites.overwrite[2].target, "Filter 1");
    assert_eq!(overwrites.overwrite[3].universal, "Universal Wheel 1.Universal Wheel Slot 2");

    let connections = o.connections.as_ref().expect("Expected fixture connections");
    assert_eq!(connections.connection.len(), 3);
    assert_eq!(connections.connection[0].own, "Input");
    assert_eq!(connections.connection[0].other, "Output1");
    assert_eq!(connections.connection[0].to_object, "deadbeef-0000-0000-0000-000000000005");
    assert_eq!(connections.connection[1].own, "1");
    assert_eq!(connections.connection[1].other, "IN");
    assert_eq!(connections.connection[2].own, "2");
    assert_eq!(connections.connection[2].other, "IN");

    assert_eq!(o.color.as_deref(), Some("0.314303,0.328065,87.699166"));

    let mappings = o.mappings.as_ref().expect("Expected mappings");
    assert_eq!(mappings.mapping.len(), 1);
    assert_eq!(mappings.mapping[0].linked_def, "deadbeef-0000-0000-0000-000000000073");
    assert_eq!(mappings.mapping[0].ux, Some(10));
    assert_eq!(mappings.mapping[0].uy, Some(10));
    assert_eq!(mappings.mapping[0].ox, Some(5));
    assert_eq!(mappings.mapping[0].oy, Some(5));
    assert_eq!(mappings.mapping[0].rz, Some(45.0));

    let gobo = o.gobo.as_ref().expect("Expected gobo");
    assert_eq!(gobo.rotation, 32.5);
    assert_eq!(gobo.file_name, "gobo.png");

    let child_list = o.child_list.as_ref().expect("Expected fixture child list");
    assert_eq!(child_list.content.len(), 1);
    let ChildListContent::FocusPoint(child_fp) = &child_list.content[0] else {
        panic!("Expected FocusPoint as Fixture child");
    };
    assert_eq!(child_fp.uuid, "deadbeef-0000-0000-0000-000000000012");
    assert_eq!(
        child_fp.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{3000.0,500.0,0.0}")
    );
    assert_eq!(child_fp.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
}

#[test]
fn test_complete_truss_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::Truss(o) if o.name == "Complete Truss 1" => Some(o),
            _ => None,
        },
        "Complete Truss 1",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000013");
    assert_eq!(o.multipatch, "");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{4000.0,0.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(o.position.as_deref(), Some("deadbeef-0000-0000-0000-000000000071"));
    assert_eq!(o.function.as_deref(), Some("This truss is meant for testing"));
    assert_eq!(o.gdtf_spec.as_deref(), Some("Robe Lighting@Robin Spiider.gdtf"));
    assert_eq!(o.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(o.cast_shadow, Some(true));
    assert_eq!(o.child_position.as_deref(), Some("Base.Yoke"));
    assert_eq!(o.fixture_id, "TRUSS-0013");
    assert_eq!(o.fixture_id_numeric, Some(1013));
    assert_eq!(o.unit_number, Some(2013));
    assert_eq!(o.custom_id_type, Some(3013));
    assert_eq!(o.custom_id, Some(4013));
    assert_eq!(o.fixture_type_id, Some(1));

    let addresses = o.addresses.as_ref().expect("Expected truss addresses");
    assert_eq!(addresses.address.len(), 1);
    assert_eq!(addresses.address[0].r#break, 0);
    assert_eq!(addresses.address[0].content, "46");
    assert_eq!(addresses.network.len(), 3);
    assert_eq!(addresses.network[0].geometry, "ethernet_1");
    assert_eq!(addresses.network[0].ipv_4.as_deref(), Some("192.168.11.6"));
    assert_eq!(addresses.network[0].subnetmask.as_deref(), Some("255.255.0.0"));
    assert!(addresses.network[0].ipv_6.is_none());
    assert_eq!(addresses.network[1].geometry, "ethernet_2");
    assert_eq!(
        addresses.network[1].ipv_6.as_deref(),
        Some("2001:0db8:85a3:0000:0000:8a2e:0370:7345")
    );
    assert!(addresses.network[1].ipv_4.is_none());
    assert_eq!(addresses.network[2].geometry, "wireless_1");
    assert_eq!(addresses.network[2].dhcp.as_deref(), Some("on"));

    let alignments = o.alignments.as_ref().expect("Expected truss alignments");
    assert_eq!(alignments.alignment.len(), 1);
    assert_eq!(alignments.alignment[0].geometry.as_deref(), Some("Beam"));
    assert_eq!(alignments.alignment[0].up, "0,0,1");
    assert_eq!(alignments.alignment[0].direction, "0,0,-1");

    let custom_commands = o.custom_commands.as_ref().expect("Expected truss custom_commands");
    assert_eq!(custom_commands.custom_command.len(), 2);
    assert_eq!(custom_commands.custom_command[0], "Body_Pan,f 50");
    assert_eq!(custom_commands.custom_command[1], "Yoke_Tilt,f 50");

    let overwrites = o.overwrites.as_ref().expect("Expected truss overwrites");
    assert_eq!(overwrites.overwrite.len(), 4);
    assert_eq!(overwrites.overwrite[0].universal, "Universal Wheel 1.Universal Wheel Slot 1");
    assert_eq!(overwrites.overwrite[0].target, "Wheel 1.Wheel Slot");
    assert_eq!(overwrites.overwrite[3].universal, "Universal Wheel 1.Universal Wheel Slot 2");

    let connections = o.connections.as_ref().expect("Expected truss connections");
    assert_eq!(connections.connection.len(), 3);
    assert_eq!(connections.connection[0].own, "Input");
    assert_eq!(connections.connection[0].other, "Output1");
    assert_eq!(connections.connection[0].to_object, "deadbeef-0000-0000-0000-000000000005");

    let child_list = o.child_list.as_ref().expect("Expected truss child list");
    assert_eq!(child_list.content.len(), 1);
    let ChildListContent::SceneObject(child) = &child_list.content[0] else {
        panic!("Expected SceneObject as Truss child");
    };
    assert_eq!(child.uuid, "deadbeef-0000-0000-0000-000000000014");
    assert_eq!(child.geometries.symbol.len(), 1);
    assert_eq!(
        child.geometries.symbol[0].matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{500.0,0.0,0.0}")
    );
}

#[test]
fn test_complete_support_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::Support(o) if o.name == "Complete Support 1" => Some(o),
            _ => None,
        },
        "Complete Support 1",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000015");
    assert_eq!(o.multipatch, "");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{5000.0,0.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(o.position.as_deref(), Some("deadbeef-0000-0000-0000-000000000071"));
    assert_eq!(o.function.as_deref(), Some("This support is meant for testing"));
    assert_eq!(o.chain_length, 2.5);
    assert_eq!(o.gdtf_spec.as_deref(), Some("Robe Lighting@Robin Spiider.gdtf"));
    assert_eq!(o.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(o.cast_shadow, Some(true));
    assert_eq!(o.fixture_id, "SUPPORT-0015");
    assert_eq!(o.fixture_id_numeric, Some(1015));
    assert_eq!(o.unit_number, Some(2015));
    assert_eq!(o.custom_id_type, Some(3015));
    assert_eq!(o.custom_id, Some(4015));
    assert_eq!(o.fixture_type_id, Some(1));

    let addresses = o.addresses.as_ref().expect("Expected support addresses");
    assert_eq!(addresses.address.len(), 1);
    assert_eq!(addresses.address[0].r#break, 0);
    assert_eq!(addresses.address[0].content, "50");

    let alignments = o.alignments.as_ref().expect("Expected support alignments");
    assert_eq!(alignments.alignment.len(), 1);
    assert_eq!(alignments.alignment[0].geometry.as_deref(), Some("SupportBeam"));
    assert_eq!(alignments.alignment[0].up, "0,0,1");
    assert_eq!(alignments.alignment[0].direction, "1,0,0");

    let custom_commands = o.custom_commands.as_ref().expect("Expected support custom_commands");
    assert_eq!(custom_commands.custom_command.len(), 1);
    assert_eq!(custom_commands.custom_command[0], "Support_Lift,f 100");

    let overwrites = o.overwrites.as_ref().expect("Expected support overwrites");
    assert_eq!(overwrites.overwrite.len(), 1);
    assert_eq!(overwrites.overwrite[0].universal, "Universal Support");
    assert_eq!(overwrites.overwrite[0].target, "Support Target");

    let connections = o.connections.as_ref().expect("Expected support connections");
    assert_eq!(connections.connection.len(), 1);
    assert_eq!(connections.connection[0].own, "SupportBase");
    assert_eq!(connections.connection[0].other, "TrussBase");
    assert_eq!(connections.connection[0].to_object, "deadbeef-0000-0000-0000-000000000013");

    let child_list = o.child_list.as_ref().expect("Expected support child list");
    assert_eq!(child_list.content.len(), 1);
    let ChildListContent::SceneObject(child) = &child_list.content[0] else {
        panic!("Expected SceneObject as Support child");
    };
    assert_eq!(child.uuid, "deadbeef-0000-0000-0000-000000000016");
    assert_eq!(child.name, "Support Child");
}

#[test]
fn test_complete_video_screen_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::VideoScreen(o) if o.name == "Complete VideoScreen 1" => Some(o),
            _ => None,
        },
        "Complete VideoScreen 1",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000017");
    assert_eq!(o.multipatch, "");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{6000.0,0.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(o.function.as_deref(), Some("This video screen is meant for testing"));
    assert_eq!(o.gdtf_spec.as_deref(), Some("Robe Lighting@Robin Spiider.gdtf"));
    assert_eq!(o.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(o.cast_shadow, Some(true));
    assert_eq!(o.fixture_id, "SCREEN-0017");
    assert_eq!(o.fixture_id_numeric, Some(1017));
    assert_eq!(o.unit_number, Some(2017));
    assert_eq!(o.custom_id_type, Some(3017));
    assert_eq!(o.custom_id, Some(4017));
    assert_eq!(o.fixture_type_id, Some(1));

    let sources = o.sources.as_ref().expect("Expected sources");
    assert_eq!(sources.source.len(), 1);
    assert_eq!(sources.source[0].linked_geometry, "Display1");
    assert_eq!(sources.source[0].r#type, SourceType::File);
    assert_eq!(sources.source[0].content, "movie.mov");

    let addresses = o.addresses.as_ref().expect("Expected video_screen addresses");
    assert_eq!(addresses.address.len(), 1);
    assert_eq!(addresses.address[0].r#break, 0);
    assert_eq!(addresses.address[0].content, "51");

    let alignments = o.alignments.as_ref().expect("Expected video_screen alignments");
    assert_eq!(alignments.alignment.len(), 1);
    assert_eq!(alignments.alignment[0].geometry.as_deref(), Some("ScreenSurface"));
    assert_eq!(alignments.alignment[0].up, "0,1,0");
    assert_eq!(alignments.alignment[0].direction, "0,0,-1");

    let custom_commands =
        o.custom_commands.as_ref().expect("Expected video_screen custom_commands");
    assert_eq!(custom_commands.custom_command.len(), 1);
    assert_eq!(custom_commands.custom_command[0], "Screen_Brightness,f 100");

    let overwrites = o.overwrites.as_ref().expect("Expected video_screen overwrites");
    assert_eq!(overwrites.overwrite.len(), 1);
    assert_eq!(overwrites.overwrite[0].universal, "Universal Screen");
    assert_eq!(overwrites.overwrite[0].target, "Screen Target");

    let connections = o.connections.as_ref().expect("Expected video_screen connections");
    assert_eq!(connections.connection.len(), 1);
    assert_eq!(connections.connection[0].own, "ScreenInput");
    assert_eq!(connections.connection[0].other, "ProjectorOutput");
    assert_eq!(connections.connection[0].to_object, "deadbeef-0000-0000-0000-000000000018");

    let child_list = o.child_list.as_ref().expect("Expected video_screen child list");
    assert_eq!(child_list.content.len(), 1);
    let ChildListContent::SceneObject(child) = &child_list.content[0] else {
        panic!("Expected SceneObject as VideoScreen child");
    };
    assert_eq!(child.uuid, "deadbeef-0000-0000-0000-000000000019");
    assert_eq!(child.name, "VideoScreen Child");
}

#[test]
fn test_complete_projector_fields() {
    let mvr = load_complete_mvr();

    let o = find_complete_layer_object(
        &mvr,
        |i| match i {
            ChildListContent::Projector(o) if o.name == "Complete Projector 1" => Some(o),
            _ => None,
        },
        "Complete Projector 1",
    );

    assert_eq!(o.uuid, "deadbeef-0000-0000-0000-000000000018");
    assert_eq!(o.multipatch, "");
    assert_eq!(
        o.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{7000.0,0.0,0.0}")
    );
    assert_eq!(o.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(o.gdtf_spec.as_deref(), Some("ProjectorFixture.gdtf"));
    assert_eq!(o.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(o.fixture_id, "PROJECTOR-0018");
    assert_eq!(o.fixture_id_numeric, Some(1018));
    assert_eq!(o.unit_number, Some(2018));
    assert_eq!(o.custom_id_type, Some(3018));
    assert_eq!(o.custom_id, Some(4018));
    assert_eq!(o.fixture_type_id, Some(1));

    assert_eq!(o.projections.projection.len(), 1);
    let proj = &o.projections.projection[0];
    assert_eq!(proj.source.len(), 1);
    assert_eq!(proj.source[0].linked_geometry, "Beam1");
    assert_eq!(proj.source[0].r#type, SourceType::File);
    assert_eq!(proj.source[0].content, "projector_content.mov");
    assert_eq!(proj.scale_handeling.len(), 1);
    assert_eq!(proj.scale_handeling[0].r#enum, Scale::ScaleKeepRatio);

    let addresses = o.addresses.as_ref().expect("Expected projector addresses");
    assert_eq!(addresses.address.len(), 1);
    assert_eq!(addresses.address[0].r#break, 0);
    assert_eq!(addresses.address[0].content, "52");

    let alignments = o.alignments.as_ref().expect("Expected projector alignments");
    assert_eq!(alignments.alignment.len(), 1);
    assert_eq!(alignments.alignment[0].geometry.as_deref(), Some("Beam"));
    assert_eq!(alignments.alignment[0].up, "0,1,0");
    assert_eq!(alignments.alignment[0].direction, "0,0,-1");

    let custom_commands = o.custom_commands.as_ref().expect("Expected projector custom_commands");
    assert_eq!(custom_commands.custom_command.len(), 1);
    assert_eq!(custom_commands.custom_command[0], "Projector_Zoom,f 100");

    let overwrites = o.overwrites.as_ref().expect("Expected projector overwrites");
    assert_eq!(overwrites.overwrite.len(), 1);
    assert_eq!(overwrites.overwrite[0].universal, "Universal Projector");
    assert_eq!(overwrites.overwrite[0].target, "Projector Target");

    let connections = o.connections.as_ref().expect("Expected projector connections");
    assert_eq!(connections.connection.len(), 1);
    assert_eq!(connections.connection[0].own, "ProjectorOutput");
    assert_eq!(connections.connection[0].other, "ScreenInput");
    assert_eq!(connections.connection[0].to_object, "deadbeef-0000-0000-0000-000000000017");

    let child_list = o.child_list.as_ref().expect("Expected projector child list");
    assert_eq!(child_list.content.len(), 1);
    let ChildListContent::SceneObject(child) = &child_list.content[0] else {
        panic!("Expected SceneObject as Projector child");
    };
    assert_eq!(child.uuid, "deadbeef-0000-0000-0000-000000000020");
    assert_eq!(child.name, "Projector Child");
}

#[test]
fn test_mvr_bundle_from_archive() {
    let archive_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("complete_mvr.mvr");

    let mvr = Mvr::from_archive(archive_path);
    let desc = mvr.bundle().description();

    assert_eq!(desc.provider, Some("Handwritten".to_string()));
    assert_eq!(desc.ver_major, 1);
    assert_eq!(desc.ver_minor, 6);
    assert_eq!(desc.scene.layers.layer.len(), 3);

    let aux = desc.scene.aux_data.as_ref().expect("Expected AUXData");
    assert_eq!(aux.symdef.len(), 5);
}

#[test]
fn test_mvr_bundle_resources() {
    let mvr = load_complete_mvr();
    let resources = mvr.bundle().resources();

    let gdtf_key = ResourceKey::new("Robe Lighting@Robin Spiider.gdtf");
    let gdtf_entry = resources.get(&gdtf_key).expect("Expected GDTF resource entry");
    assert_eq!(gdtf_entry.kind, ResourceKind::Gdtf);

    let xml_key = ResourceKey::new("GeneralSceneDescription.xml");
    let xml_entry = resources.get(&xml_key).expect("Expected XML resource entry");
    assert_eq!(xml_entry.kind, ResourceKind::Other);

    let missing_key = ResourceKey::new("nonexistent.glb");
    assert!(!resources.contains_key(&missing_key));
}
