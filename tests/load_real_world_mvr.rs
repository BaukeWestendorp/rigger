//! Loads the real-world MVR files listed in `tests/data/mvr/real-world/manifest.tsv`.
//!
//! These files are not committed. Fetch them first:
//!
//! ```sh
//! scripts/fetch-real-world-mvrs.sh
//! cargo test --test load_real_world_mvr -- --ignored
//! ```

use std::{
    panic,
    path::{Path, PathBuf},
};

use rigger::mvr::{
    Mvr,
    layer::{Object, ObjectKind},
};

struct Entry {
    file: String,
    fixtures: usize,
    unresolved_gdtfs: Vec<String>,
}

fn real_world_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/mvr/real-world")
}

fn manifest() -> Vec<Entry> {
    let manifest = std::fs::read_to_string(real_world_dir().join("manifest.tsv")).unwrap();
    manifest
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let columns: Vec<&str> = line.split('\t').collect();
            let unresolved_gdtfs = match columns[3] {
                "-" => Vec::new(),
                specs => specs.split(',').map(str::to_owned).collect(),
            };
            Entry {
                file: columns[0].to_owned(),
                fixtures: columns[2].parse().unwrap(),
                unresolved_gdtfs,
            }
        })
        .collect()
}

fn walk(mvr: &Mvr, objects: &[Object], fixtures: &mut usize, unresolved: &mut Vec<String>) {
    for object in objects {
        if matches!(object.kind(), ObjectKind::Fixture(_)) {
            *fixtures += 1;
        }

        if let Some(info) = object.gdtf_info()
            && mvr.resources().gdtf(info.resource()).is_none()
        {
            let spec = info.resource().relative_path().display().to_string();
            if !unresolved.contains(&spec) {
                unresolved.push(spec);
            }
        }

        if let Some(children) = object.child_objects() {
            walk(mvr, children, fixtures, unresolved);
        }
    }
}

#[test]
#[ignore = "needs the files from scripts/fetch-real-world-mvrs.sh"]
fn test_load_real_world_mvrs() {
    let mut failures = Vec::new();

    for entry in manifest() {
        let path = real_world_dir().join(&entry.file);
        assert!(
            path.exists(),
            "{} is missing, run scripts/fetch-real-world-mvrs.sh",
            path.display()
        );

        let result = panic::catch_unwind(|| {
            let mvr = Mvr::from_archive(&path);
            let mut fixtures = 0;
            let mut unresolved = Vec::new();
            for layer in mvr.layers() {
                walk(&mvr, layer.objects(), &mut fixtures, &mut unresolved);
            }
            (fixtures, unresolved)
        });

        match result {
            Ok((fixtures, unresolved))
                if fixtures == entry.fixtures && unresolved == entry.unresolved_gdtfs => {}
            Ok((fixtures, unresolved)) => failures.push(format!(
                "{}: expected {} fixtures and unresolved GDTFs {:?}, got {} and {:?}",
                entry.file, entry.fixtures, entry.unresolved_gdtfs, fixtures, unresolved
            )),
            Err(_) => failures.push(format!("{}: panicked while loading", entry.file)),
        }
    }

    assert!(failures.is_empty(), "failed to load real-world MVRs:\n{}", failures.join("\n"));
}
