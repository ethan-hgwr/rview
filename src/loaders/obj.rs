use crate::loaders::{Loader, Mesh};
use anyhow::{Result, anyhow};
use rayon::prelude::*;
use tobj::load_obj;
use std::path::Path;

pub(crate) struct ObjLoader;

impl Loader for ObjLoader {
    fn load(path: &Path) -> Result<Mesh> {
        let (models, _) = load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )?;

        if models.is_empty() {
            return Err(anyhow!("OBJ file contains no models"));
        }

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in &models {
            let mesh = &model.mesh;
            let vertex_offset = vertices.len();

            if mesh.positions.is_empty() {
                continue;
            }

            if !mesh.indices.len().is_multiple_of(3) {
                return Err(anyhow!(
                    "model '{}' indices are not a multiple of 3, please triangulate your model.",
                    model.name
                ));
            }

            let new_verts: Vec<[f32; 3]> = mesh
                .positions
                .par_chunks_exact(3)
                .map(|p| [p[0], p[1], p[2]])
                .collect();

            let new_indices: Vec<[usize; 3]> = mesh
                .indices
                .par_chunks_exact(3)
                .map(|f| {
                    [
                        vertex_offset + f[0] as usize,
                        vertex_offset + f[1] as usize,
                        vertex_offset + f[2] as usize,
                    ]
                })
                .collect();

            vertices.extend(new_verts);
            indices.extend(new_indices);
        }

        if vertices.is_empty() {
            return Err(anyhow!("OBJ file contains no geometry"));
        }

        Ok(Mesh { vertices, indices })
    }
}
