use std::path::Path;

pub mod gdtf;
pub mod mvr;

pub use ::glam;

fn sanetize_path(path: impl AsRef<Path>) -> String {
    use unicode_normalization::UnicodeNormalization;
    let path = path.as_ref();
    let mut out = String::new();
    for component in path.components() {
        let std::path::Component::Normal(part) = component else { continue };

        let part = part.to_string_lossy().nfc().to_string();
        if out.is_empty() {
            out.push_str(&part);
        } else {
            out.push('/');
            out.push_str(&part);
        }
    }
    out
}
