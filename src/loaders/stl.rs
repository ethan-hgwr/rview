use anyhow::{Result, anyhow};
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::path::Path;

use crate::loaders::{Loader, Mesh};

pub(crate) struct StlLoader;

impl Loader for StlLoader {
    fn load(path: &Path) -> Result<Mesh> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        let stl = stl_io::create_stl_reader(&mut file)
            .map_err(|e| anyhow!("failed to read STL: {e}"))?
            .as_indexed_triangles()
            .map_err(|e| anyhow!("failed to index STL triangles: {e}"))?;

        let vertices: Vec<[f32; 3]> = stl
            .vertices
            .par_iter()
            .map(|v| [v[0], v[1], v[2]])
            .collect();

        let indices: Vec<[usize; 3]> = stl
            .faces
            .par_iter()
            .map(|f| [f.vertices[0], f.vertices[1], f.vertices[2]])
            .collect();

        Ok(Mesh { vertices, indices })
    }
}
