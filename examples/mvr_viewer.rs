use std::{collections::HashMap, io::Write as _};

use raylib::prelude::*;
use rigger::mvr::{Layer, Mvr, Object, ResourceKey};

struct State {
    pub mvr: Mvr,
    pub tempdir: tempfile::TempDir,

    pub camera: Camera,
    pub models: HashMap<ResourceKey, Model>,
    pub textures: HashMap<ResourceKey, Texture2D>,
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <path>", std::env::args().next().unwrap());
            std::process::exit(1);
        }
    };

    let mvr = {
        let start = std::time::Instant::now();
        let mvr = rigger::mvr::Mvr::from_archive(path);
        let duration = start.elapsed();
        println!("Loaded MVR in {:?}", duration);
        mvr
    };

    run(mvr);
}

fn run(mvr: Mvr) {
    let (mut rl, mut thread) = raylib::init()
        .size(1080, 720)
        .title("MVR Viewer")
        .msaa_4x()
        .resizable()
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let mut state = State {
        mvr,
        tempdir: tempfile::tempdir().unwrap(),
        camera: Camera::perspective(
            Vector3::new(0.0, 5.0, -10.0), // Position
            Vector3::new(0.0, 1.0, 1.0),   // Target
            Vector3::new(0.0, 1.0, 0.0),   // Up vector
            90.0,                          // FOV
        ),
        models: HashMap::new(),
        textures: HashMap::new(),
    };

    setup(&mut state, &mut thread, &mut rl);
    while !rl.window_should_close() {
        update(&mut state, &mut thread, &mut rl);
        draw(&mut state, &mut thread, &mut rl);
    }
}

fn setup(state: &mut State, thread: &RaylibThread, rl: &mut RaylibHandle) {
    for (key, texture) in state.mvr.resources().textures() {
        let texture_bytes = texture.bytes();
        let temp_texture_path = state.tempdir.path().join(key.relative_path());
        let mut temp_file = std::fs::File::create(&temp_texture_path).unwrap();
        temp_file.write_all(&texture_bytes).unwrap();
        let path_str = temp_texture_path.to_string_lossy().to_string();

        let rl_texture = rl.load_texture(thread, path_str.as_str()).unwrap();
        state.textures.insert(key.clone(), rl_texture);
    }

    for (key, model) in state.mvr.resources().models() {
        let model_bytes = model.bytes();
        let temp_model_path = state.tempdir.path().join(key.relative_path());
        let mut temp_file = std::fs::File::create(&temp_model_path).unwrap();
        temp_file.write_all(&model_bytes).unwrap();
        let path_str = temp_model_path.to_string_lossy().to_string();

        let rl_model = rl.load_model(thread, path_str.as_str()).unwrap();
        state.models.insert(key.clone(), rl_model);
    }
}

fn update(state: &mut State, _thread: &RaylibThread, rl: &mut RaylibHandle) {
    rl.update_camera(&mut state.camera, CameraMode::CAMERA_FREE);
}

fn draw(state: &mut State, thread: &RaylibThread, rl: &mut RaylibHandle) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::RAYWHITE);
    d.draw_mode3D(state.camera, |mut d, camera| {
        d.draw_grid(10, 1.0);

        unsafe { raylib::ffi::rlDisableBackfaceCulling() };
        d.draw_mode3D(camera, |mut d, _| {
            for layer in state.mvr.layers() {
                let layer_world = layer.local_transform();
                draw_layer(layer, layer_world, state, &mut d);
            }
        });
        unsafe { raylib::ffi::rlEnableBackfaceCulling() };
    });
    d.draw_fps(10, 10);
}

fn to_world(parent_world: glam::Affine3A, local: glam::Affine3A) -> glam::Affine3A {
    parent_world * local
}

fn draw_layer(
    layer: &Layer,
    layer_world: glam::Affine3A,
    state: &State,
    d: &mut RaylibMode3D<RaylibDrawHandle<'_>>,
) {
    fn draw_objects(
        objects: &[Object],
        parent_world: glam::Affine3A,
        state: &State,
        d: &mut RaylibMode3D<RaylibDrawHandle<'_>>,
    ) {
        for object in objects {
            let object_world = to_world(parent_world, object.local_transform());
            draw_object(object, object_world, state, d);
            if let Some(objects) = object.child_objects() {
                draw_objects(objects, object_world, state, d);
            }
        }
    }

    draw_objects(layer.objects(), layer_world, state, d);
}

fn draw_object(
    object: &Object,
    object_world: glam::Affine3A,
    state: &State,
    d: &mut RaylibMode3D<RaylibDrawHandle<'_>>,
) {
    if let Some(fixture) = object.as_fixture_object() {
        let position = convert_coordinate_space(object_world).translation;
        match fixture.gdtf() {
            Some(gdtf_info) => {
                let _gdtf = state.mvr.resources().gdtf(gdtf_info.gdtf_resource()).unwrap();
            }
            None => {
                d.draw_sphere(Vector3::new(position.x, position.y, position.z), 0.1, Color::RED);
            }
        }
    }

    let Some(geometries) = object.geometries() else {
        return;
    };

    for geometry in geometries {
        let model = state.models.get(&geometry.model()).unwrap();
        let geometry_world = to_world(object_world, geometry.local_transform());
        draw_model(model, geometry_world, d);
    }
}

fn draw_model(
    model: &Model,
    transform: glam::Affine3A,
    d: &mut RaylibMode3D<RaylibDrawHandle<'_>>,
) {
    let affine = convert_coordinate_space(transform);
    let (scale, rotation, translation) = affine.to_scale_rotation_translation();
    let translation = Vector3::new(translation.x, translation.y, translation.z);
    let (axis, angle) = rotation.to_axis_angle();
    let rotation_axis = Vector3::new(axis.x, axis.y, axis.z);
    let rotation_angle_deg = angle.to_degrees();
    let scale = Vector3::new(scale.x, scale.y, scale.z);

    d.draw_model_ex(model, translation, rotation_axis, rotation_angle_deg, scale, Color::WHITE);
}

fn convert_coordinate_space(source_transform: glam::Affine3A) -> glam::Affine3A {
    let (scale, rotation, translation) = source_transform.to_scale_rotation_translation();
    let new_translation = glam::Vec3::new(translation.x, translation.z, translation.y);
    let (axis, angle) = rotation.to_axis_angle();
    let new_axis = glam::Vec3::new(-axis.x, -axis.z, -axis.y);
    let new_rotation = glam::Quat::from_axis_angle(new_axis, angle);
    let new_scale = glam::Vec3::new(scale.x, scale.y, -scale.z);
    glam::Affine3A::from_scale_rotation_translation(new_scale, new_rotation, new_translation)
}
