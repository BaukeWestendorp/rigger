use rigger::gdtf::Gdtf;

// These are some badly serialized GDTF descriptions.
const BLACKLIST: &[&str] = &[
    "ARRI@Orbiter@DMX_v4.5_29_Dec_Standard_and_ECC.gdtf", // PhysicalUnit = "Temperatur"
    "GDTF_hed@Basic_Conventional@honza.gdtf",             // Missing DMXMode channels
    "MegaLite@FrostBot@Initial_upload_by_GDTFservice_powered_by_Q2Q_Technologies.gdtf", // ColorRenderingIndex="6631" (Size should be 1 byte).
    "Lightline@LaserLink@Gobo_PNGs_added_May_2020.gdtf", // Duplicate <Revisions>
    "ACME@ACME_XP-260_BEAM@ACME_XP-260_BEAM.gdtf",       // Duplicate <Revisions>
    "PROLIGHT_SPAIN@PAR_PRO_270_5-IN-1@Created.gdtf",    // Duplicate <Revisions>
    "User_Test@ZZFixture2@asdfasdasdfasdf_sdfasdfasdfasdf.gdtf", // Duplicate <Revisions>
    "Vari-Lite@VL2600_Profile@V3.0.gdtf",                // Duplicate <Revisions>
];

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <path>", std::env::args().next().unwrap());
            std::process::exit(1);
        }
    };

    let entries = match std::fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", path, e);
            std::process::exit(1);
        }
    };

    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.path());
    let total = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let gdtf_path = entry.path();

        if let Some(file_name) = gdtf_path.file_name().and_then(|f| f.to_str()) {
            if BLACKLIST.iter().any(|&b| b == file_name) {
                continue;
            }
        }

        if gdtf_path.extension().and_then(|s| s.to_str()) == Some("gdtf") {
            eprintln!("PARSING ({}/{}): {}", i + 1, total, gdtf_path.display());
            let _ = Gdtf::from_archive(&gdtf_path);
        }
    }
}
