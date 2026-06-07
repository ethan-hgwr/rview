use anyhow::Result;
use std::path::Path;

use crate::loaders::{gltf::GltfLoader, obj::ObjLoader, stl::StlLoader};

pub(crate) mod gltf;
pub(crate) mod obj;
pub(crate) mod stl;

pub(crate) struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<[usize; 3]>,
}

pub(crate) trait Loader {
    fn load(path: &Path) -> Result<Mesh>;
}

pub(crate) fn load(path: &str) -> Result<Mesh> {
    let path = Path::new(path);

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| anyhow::anyhow!("file has no extension"))?;

    match extension {
        "obj" => ObjLoader::load(path),
        "gltf" | "glb" => GltfLoader::load(path),
        "stl" => StlLoader::load(path),
        ext => Err(anyhow::anyhow!("unsupported format: .{ext}")),
    }
}
