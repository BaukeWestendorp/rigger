use std::path::Path;

use rigger::mvr::Mvr;
use uuid::Uuid;

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
