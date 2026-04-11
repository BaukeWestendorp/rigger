use std::path::Path;

pub mod mvr;

pub mod stage;

pub fn sanetize_file_name(file_name: impl AsRef<Path>) -> String {
    use unicode_normalization::UnicodeNormalization;
    file_name.as_ref().file_name().unwrap().to_string_lossy().nfc().to_string()
}
