use std::path::Path;

use rigger::mvr::{
    Mvr,
    bundle::{ChildListContent, Scale, SourceType, Transmission},
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
    assert_eq!(symdef_1.child_list.symbol.len(), 0);

    let symdef_2 = aux
        .symdef
        .iter()
        .find(|s| s.uuid == "deadbeef-0000-0000-0000-000000000080")
        .expect("Expected Symdef 'Symbol 2'");
    assert_eq!(symdef_2.name, "Symbol 2");
    assert_eq!(symdef_2.child_list.geometry_3d.len(), 1);
    assert_eq!(symdef_2.child_list.geometry_3d[0].file_name, "model2.glb");
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

#[test]
fn test_mvr_bundle_layer_objects_complete_layer_parses_all_fields() {
    let mvr = load_complete_mvr();
    let desc = mvr.bundle().description();

    let layer = desc
        .scene
        .layers
        .layer
        .iter()
        .find(|l| l.name == "Complete Object Layer")
        .expect("Expected 'Complete Object Layer'");

    let child_list = layer.child_list.as_ref().expect("Expected ChildList in complete layer");

    let mut scene_object = None;
    let mut group_object = None;
    let mut focus_point = None;
    let mut fixture = None;
    let mut truss = None;
    let mut support = None;
    let mut video_screen = None;
    let mut projector = None;

    for item in &child_list.content {
        match item {
            ChildListContent::SceneObject(o) if o.name == "Complete SceneObject 1" => {
                scene_object = Some(o);
            }
            ChildListContent::GroupObject(o) if o.name == "Complete Group 1" => {
                group_object = Some(o);
            }
            ChildListContent::FocusPoint(o) if o.name == "Complete Point 2" => {
                focus_point = Some(o);
            }
            ChildListContent::Fixture(o) if o.name == "Complete Fixture 1" => {
                fixture = Some(o);
            }
            ChildListContent::Truss(o) if o.name == "Complete Truss 1" => {
                truss = Some(o);
            }
            ChildListContent::Support(o) if o.name == "Complete Support 1" => {
                support = Some(o);
            }
            ChildListContent::VideoScreen(o) if o.name == "Complete VideoScreen 1" => {
                video_screen = Some(o);
            }
            ChildListContent::Projector(o) if o.name == "Complete Projector 1" => {
                projector = Some(o);
            }
            _ => {}
        }
    }

    let scene_object = scene_object.expect("Expected Complete SceneObject 1");
    assert_eq!(scene_object.uuid, "deadbeef-0000-0000-0000-000000000005");
    assert_eq!(scene_object.multipatch, "");
    assert_eq!(
        scene_object.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{1000.0,0.0,0.0}")
    );
    assert_eq!(scene_object.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(scene_object.gdtf_spec.as_deref(), Some("Robe Lighting@Robin Spiider.gdtf"));
    assert_eq!(scene_object.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(scene_object.cast_shadow, Some(true));

    let addresses = scene_object.addresses.as_ref().expect("Expected addresses");
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

    let alignments = scene_object.alignments.as_ref().expect("Expected alignments");
    assert_eq!(alignments.alignment.len(), 1);
    assert_eq!(alignments.alignment[0].geometry.as_deref(), Some("Beam1"));
    assert_eq!(alignments.alignment[0].up, "0,0,1");
    assert_eq!(alignments.alignment[0].direction, "0,0,-1");

    let overwrites = scene_object.overwrites.as_ref().expect("Expected overwrites");
    assert_eq!(overwrites.overwrite.len(), 1);
    assert_eq!(overwrites.overwrite[0].universal, "SomeUniversalNode");
    assert_eq!(overwrites.overwrite[0].target, "SomeTargetNode");

    let connections = scene_object.connections.as_ref().expect("Expected connections");
    assert_eq!(connections.connection.len(), 1);
    assert_eq!(connections.connection[0].own, "Base");
    assert_eq!(connections.connection[0].other, "Base.Yoke");
    assert_eq!(connections.connection[0].to_object, "deadbeef-0000-0000-0000-000000000009");

    assert_eq!(scene_object.fixture_id.as_deref(), Some("SCENEOBJECT-0005"));
    assert_eq!(scene_object.fixture_id_numeric, Some(1005));
    assert_eq!(scene_object.unit_number, Some(2005));
    assert_eq!(scene_object.custom_id_type, Some(3005));
    assert_eq!(scene_object.custom_id, Some(4005));

    let fixture = fixture.expect("Expected Complete Fixture 1");
    assert_eq!(fixture.uuid, "deadbeef-0000-0000-0000-000000000009");
    assert_eq!(fixture.multipatch, "");
    assert_eq!(
        fixture.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{3000.0,0.0,0.0}")
    );
    assert_eq!(fixture.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(fixture.gdtf_spec.as_deref(), Some("Robe Lighting@Robin Spiider.gdtf"));
    assert_eq!(fixture.gdtf_mode.as_deref(), Some("Mode 1 - Standard 16 bit"));
    assert_eq!(fixture.focus.as_deref(), Some("deadbeef-0000-0000-0000-000000000006"));
    assert_eq!(fixture.cast_shadow, Some(true));
    assert_eq!(fixture.dmx_invert_pan, Some(true));
    assert_eq!(fixture.dmx_invert_tilt, Some(false));
    assert_eq!(fixture.position.as_deref(), Some("deadbeef-0000-0000-0000-000000000071"));
    assert_eq!(fixture.function.as_deref(), Some("This fixture is meant for testing"));
    assert_eq!(fixture.fixture_id, "FIXTURE-0009");
    assert_eq!(fixture.fixture_id_numeric, Some(1009));
    assert_eq!(fixture.unit_number, 2009);
    assert_eq!(fixture.child_position.as_deref(), Some("Base.Yoke"));

    let fixture_protocols = fixture.protocols.as_ref().expect("Expected protocols");
    assert_eq!(fixture_protocols.protocol.len(), 6);

    assert_eq!(fixture_protocols.protocol[0].geometry, "NetworkInOut_1");
    assert_eq!(fixture_protocols.protocol[0].name, "");
    assert_eq!(fixture_protocols.protocol[0].r#type, "Art-Net");
    assert_eq!(fixture_protocols.protocol[0].transmission, None);

    assert_eq!(fixture_protocols.protocol[1].geometry, "NetworkInOut_3");
    assert_eq!(fixture_protocols.protocol[1].name, "NDI 1");
    assert_eq!(fixture_protocols.protocol[1].r#type, "NDI");
    assert_eq!(fixture_protocols.protocol[1].transmission, None);

    assert_eq!(fixture_protocols.protocol[2].transmission, Some(Transmission::Unicast));
    assert_eq!(fixture_protocols.protocol[3].transmission, Some(Transmission::Multicast));
    assert_eq!(fixture_protocols.protocol[4].transmission, Some(Transmission::Broadcast));
    assert_eq!(fixture_protocols.protocol[5].transmission, Some(Transmission::Anycast));

    let fixture_addresses = fixture.addresses.as_ref().expect("Expected fixture addresses");
    assert_eq!(fixture_addresses.address.len(), 1);
    assert_eq!(fixture_addresses.address[0].r#break, 0);
    assert_eq!(fixture_addresses.address[0].content, "45");
    assert_eq!(fixture_addresses.network.len(), 3);

    assert_eq!(fixture.color.as_deref(), Some("0.314303,0.328065,87.699166"));

    let fixture_mappings = fixture.mappings.as_ref().expect("Expected mappings");
    assert_eq!(fixture_mappings.mapping.len(), 1);
    assert_eq!(fixture_mappings.mapping[0].linked_def, "deadbeef-0000-0000-0000-000000000073");
    assert_eq!(fixture_mappings.mapping[0].ux, Some(10));
    assert_eq!(fixture_mappings.mapping[0].uy, Some(10));
    assert_eq!(fixture_mappings.mapping[0].ox, Some(5));
    assert_eq!(fixture_mappings.mapping[0].oy, Some(5));
    assert_eq!(fixture_mappings.mapping[0].rz, Some(45.0));

    let fixture_gobo = fixture.gobo.as_ref().expect("Expected gobo");
    assert_eq!(fixture_gobo.rotation, 32.5);
    assert_eq!(fixture_gobo.file_name, "gobo.png");

    let truss = truss.expect("Expected Complete Truss 1");
    assert_eq!(truss.uuid, "deadbeef-0000-0000-0000-000000000013");
    assert_eq!(truss.multipatch, "");
    assert_eq!(
        truss.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{4000.0,0.0,0.0}")
    );
    assert_eq!(truss.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(truss.position.as_deref(), Some("deadbeef-0000-0000-0000-000000000071"));
    assert_eq!(truss.function.as_deref(), Some("This truss is meant for testing"));
    assert_eq!(truss.cast_shadow, Some(true));
    assert_eq!(truss.child_position.as_deref(), Some("Base.Yoke"));
    assert_eq!(truss.fixture_id, "TRUSS-0013");
    assert_eq!(truss.fixture_id_numeric, Some(1013));
    assert_eq!(truss.unit_number, Some(2013));

    let truss_child_list = truss.child_list.as_ref().expect("Expected truss child list");
    assert_eq!(truss_child_list.content.len(), 1);

    let support = support.expect("Expected Complete Support 1");
    assert_eq!(support.uuid, "deadbeef-0000-0000-0000-000000000015");
    assert_eq!(support.multipatch, "");
    assert_eq!(
        support.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{5000.0,0.0,0.0}")
    );
    assert_eq!(support.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(support.position.as_deref(), Some("deadbeef-0000-0000-0000-000000000071"));
    assert_eq!(support.function.as_deref(), Some("This support is meant for testing"));
    assert_eq!(support.chain_length, 2.5);
    assert_eq!(support.cast_shadow, Some(true));
    assert_eq!(support.fixture_id, "SUPPORT-0015");
    assert_eq!(support.fixture_id_numeric, Some(1015));
    assert_eq!(support.unit_number, Some(2015));

    let video_screen = video_screen.expect("Expected Complete VideoScreen 1");
    assert_eq!(video_screen.uuid, "deadbeef-0000-0000-0000-000000000017");
    assert_eq!(video_screen.multipatch, "");
    assert_eq!(
        video_screen.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{6000.0,0.0,0.0}")
    );
    assert_eq!(video_screen.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    let vs_sources = video_screen.sources.as_ref().expect("Expected sources");
    assert_eq!(vs_sources.source.len(), 1);
    assert_eq!(vs_sources.source[0].linked_geometry, "Display1");
    assert_eq!(vs_sources.source[0].r#type, SourceType::File);
    assert_eq!(vs_sources.source[0].content, "movie.mov");

    let projector = projector.expect("Expected Complete Projector 1");
    assert_eq!(projector.uuid, "deadbeef-0000-0000-0000-000000000018");
    assert_eq!(projector.multipatch, "");
    assert_eq!(
        projector.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{7000.0,0.0,0.0}")
    );
    assert_eq!(projector.classing.as_deref(), Some("deadbeef-0000-0000-0000-000000000064"));
    assert_eq!(projector.projections.projection.len(), 1);
    assert_eq!(projector.projections.projection[0].source.len(), 1);
    assert_eq!(projector.projections.projection[0].source[0].linked_geometry, "Beam1");
    assert_eq!(projector.projections.projection[0].source[0].r#type, SourceType::File);
    assert_eq!(projector.projections.projection[0].source[0].content, "projector_content.mov");

    let group_object = group_object.expect("Expected Complete Group 1");
    assert_eq!(group_object.uuid, "deadbeef-0000-0000-0000-000000000007");
    assert_eq!(
        group_object.matrix.as_deref(),
        Some("{1.0,0.0,0.0}{0.0,1.0,0.0}{0.0,0.0,1.0}{2000.0,0.0,0.0}")
    );
    assert_eq!(group_object.child_list.content.len(), 2);

    let focus_point = focus_point.expect("Expected Complete Point 2");
    assert_eq!(focus_point.uuid, "deadbeef-0000-0000-0000-000000000011");
}
