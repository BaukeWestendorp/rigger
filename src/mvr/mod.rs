use std::{
    path::{Path, PathBuf},
    str,
};

use crate::{gdtf::Gdtf, mvr::bundle::FromBundle, util};

pub mod bundle;

mod aux;
mod geo;
mod layer;
mod node;
mod resource;

pub use aux::*;
pub use geo::*;
pub use layer::*;
pub use node::*;
pub use resource::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Mvr {
    version: Version,
    provider: Provider,

    symdefs: NodeContainer<Symdef>,
    classes: NodeContainer<Class>,
    mapping_definitions: NodeContainer<MappingDefinition>,
    positions: NodeContainer<Position>,

    layers: NodeContainer<Layer>,

    path: Option<PathBuf>,
    resources: Resources,
}

impl Mvr {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_folder(path))
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_archive(path))
    }

    pub fn from_archive_bytes(bytes: &[u8]) -> Self {
        Self::from(&bundle::Bundle::from_archive_bytes(bytes))
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn set_provider(&mut self, provider: Provider) {
        self.provider = provider;
    }

    pub fn symdefs(&self) -> &NodeContainer<Symdef> {
        &self.symdefs
    }

    pub fn symdefs_mut(&mut self) -> &mut NodeContainer<Symdef> {
        &mut self.symdefs
    }

    pub fn classes(&self) -> &NodeContainer<Class> {
        &self.classes
    }

    pub fn classes_mut(&mut self) -> &mut NodeContainer<Class> {
        &mut self.classes
    }

    pub fn positions(&self) -> &NodeContainer<Position> {
        &self.positions
    }

    pub fn positions_mut(&mut self) -> &mut NodeContainer<Position> {
        &mut self.positions
    }

    pub fn mapping_definitions(&self) -> &NodeContainer<MappingDefinition> {
        &self.mapping_definitions
    }

    pub fn mapping_definitions_mut(&mut self) -> &mut NodeContainer<MappingDefinition> {
        &mut self.mapping_definitions
    }

    pub fn layers(&self) -> &NodeContainer<Layer> {
        &self.layers
    }

    pub fn layers_mut(&mut self) -> &mut NodeContainer<Layer> {
        &mut self.layers
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn set_path(&mut self, path: impl Into<PathBuf>) {
        self.path = Some(path.into());
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
}

impl Default for Mvr {
    fn default() -> Self {
        Self {
            version: Version::new(1, 6),
            provider: Provider::new("Rigger", env!("CARGO_PKG_VERSION")),
            symdefs: Default::default(),
            classes: Default::default(),
            mapping_definitions: Default::default(),
            positions: Default::default(),
            layers: Default::default(),
            path: None,
            resources: Default::default(),
        }
    }
}

impl From<&bundle::Bundle> for Mvr {
    fn from(bundle: &bundle::Bundle) -> Self {
        let mut mvr = Mvr::new();

        mvr.set_version(Version::new(
            bundle.description().ver_major as u32,
            bundle.description().ver_minor as u32,
        ));

        mvr.set_provider(Provider::new(
            bundle.description().provider.clone().unwrap_or_default(),
            bundle.description().provider_version.clone().unwrap_or_default(),
        ));

        let aux_data = bundle.description().scene.aux_data.as_ref().unwrap_or({
            static EMPTY_AUX_DATA: bundle::AuxData = bundle::AuxData {
                class: Vec::new(),
                symdef: Vec::new(),
                position: Vec::new(),
                mapping_definition: Vec::new(),
            };
            &EMPTY_AUX_DATA
        });

        for class in &aux_data.class {
            let class = Class::from_bundle(&class, &bundle);
            mvr.classes_mut().add(class);
        }

        for position in &aux_data.position {
            let position = Position::from_bundle(&position, &bundle);
            mvr.positions_mut().add(position);
        }

        for symdef in &aux_data.symdef {
            let symdef = Symdef::from_bundle(&symdef, &bundle);
            mvr.symdefs_mut().add(symdef);
        }

        for mapping_definition in &aux_data.mapping_definition {
            let mapping_definition = MappingDefinition::from_bundle(&mapping_definition, &bundle);
            mvr.mapping_definitions_mut().add(mapping_definition);
        }

        for layer in &bundle.description().scene.layers.layer {
            let layer = Layer::from_bundle(&layer, &bundle);
            mvr.layers_mut().add(layer);
        }

        for (file_name, bytes) in bundle.resources() {
            match file_name.extension().unwrap().to_str().unwrap() {
                "gdtf" => {
                    let gdtf = Gdtf::from_archive_bytes(&bytes);
                    mvr.resources_mut().add_gdtf(ResourceKey::new(file_name), gdtf);
                }
                "3ds" | "gltf" | "glb" => {
                    mvr.resources_mut().add_model(
                        ResourceKey::new(file_name),
                        ModelResource::new(file_name, bytes.clone()),
                    );
                }
                "png" | "jpg" | "jpeg" | "svg" => {
                    mvr.resources_mut().add_texture(
                        ResourceKey::new(file_name),
                        TextureResource::new(file_name, bytes.clone()),
                    );
                }
                _ => {}
            };
        }

        if let Some(path) = bundle.path() {
            mvr.set_path(path);
        }

        mvr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    major: u32,
    minor: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    name: String,
    version: String,
}

impl Provider {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self { name: name.into(), version: version.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

fn build_geometries(
    geometry_3ds: &[bundle::Geometry3D],
    symbols: &[bundle::Symbol],
    bundle: &bundle::Bundle,
) -> Vec<Geometry> {
    let mut geometries = Vec::new();

    for geo3d in geometry_3ds {
        geometries.push(Geometry::from_bundle(geo3d, bundle));
    }

    let Some(aux_data) = &bundle.description().scene.aux_data else {
        return geometries;
    };

    for symbol in symbols {
        let symbol_transform = util::parse_affine3a_or_identity(symbol.matrix.as_deref());
        let symdef = aux_data.symdef.iter().find(|s| s.uuid == symbol.symdef).unwrap();
        let nested_symdef = Symdef::from_bundle(&symdef, bundle);
        for mut geo in nested_symdef.geometries().to_owned() {
            geo.set_local_transform(geo.local_transform() * symbol_transform);
            geometries.push(geo);
        }
    }

    geometries
}
