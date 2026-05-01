use std::{collections::HashMap, fmt, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelLod {
    Low,
    Default,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvgView {
    Top,
    Bottom,
    Front,
    Side,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Thumbnail,
    Wheel,
    Model(ModelLod),
    ModelSvg(SvgView),
    Texture,
    Other,
}

impl ResourceKind {
    pub fn from_path(path: &Path) -> Self {
        let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase())
        else {
            return Self::Other;
        };

        let name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");

        if name == "thumbnail" {
            return Self::Thumbnail;
        }

        let parent =
            path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()).unwrap_or("");
        let grandparent = path
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");

        match (grandparent, parent, ext.as_str()) {
            (_, "wheels", _) => Self::Wheel,
            ("models", "svg", "svg") => Self::ModelSvg(SvgView::Top),
            ("models", "svg_bottom", "svg") => Self::ModelSvg(SvgView::Bottom),
            ("models", "svg_front", "svg") => Self::ModelSvg(SvgView::Front),
            ("models", "svg_side", "svg") => Self::ModelSvg(SvgView::Side),
            ("models", "3ds_low" | "gltf_low", _) => Self::Model(ModelLod::Low),
            ("models", "3ds_high" | "gltf_high", _) => Self::Model(ModelLod::High),
            ("models", folder, _) if folder.starts_with("3ds") || folder.starts_with("gltf") => {
                Self::Model(ModelLod::Default)
            }
            _ => match ext.as_str() {
                "png" | "jpg" | "jpeg" => Self::Texture,
                _ => Self::Other,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceKey(String);

impl ResourceKey {
    pub fn new(relative_path: impl Into<String>) -> Self {
        Self(relative_path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceEntry {
    pub key: ResourceKey,
    pub kind: ResourceKind,
}

impl ResourceEntry {
    pub fn new(key: ResourceKey, kind: ResourceKind) -> Self {
        Self { key, kind }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResourceMap {
    entries: HashMap<ResourceKey, ResourceEntry>,
}

impl ResourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> impl Iterator<Item = &ResourceEntry> {
        self.entries.values()
    }

    pub fn get(&self, key: &ResourceKey) -> Option<&ResourceEntry> {
        self.entries.get(key)
    }

    pub fn contains_key(&self, key: &ResourceKey) -> bool {
        self.entries.contains_key(key)
    }

    pub(crate) fn insert(&mut self, entry: ResourceEntry) {
        self.entries.insert(entry.key.clone(), entry);
    }
}
