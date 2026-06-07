use anyhow::{Result, anyhow};
use glam::{Mat4, Vec4};
use rayon::prelude::*;
use std::path::Path;

use crate::loaders::{Loader, Mesh};

fn process_node(
    node: &gltf::Node,
    parent_transform: Mat4,
    buffers: &[gltf::buffer::Data],
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<[usize; 3]>,
) {
    let local_transform = Mat4::from_cols_array_2d(&node.transform().matrix());
    let global_transform = parent_transform * local_transform;

    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let vertex_offset = vertices.len();

            let Some(positions) = reader.read_positions() else {
                continue;
            };

            let raw_positions: Vec<[f32; 3]> = positions.collect();

            let new_verts: Vec<[f32; 3]> = raw_positions
                .par_iter()
                .map(|&p| {
                    let t = global_transform * Vec4::new(p[0], p[1], p[2], 1.0);
                    [t.x, t.y, t.z]
                })
                .collect();

            vertices.extend(new_verts);

            let Some(iter) = reader.read_indices() else {
                continue;
            };

            let raw: Vec<u32> = iter.into_u32().collect();

            if !raw.len().is_multiple_of(3) {
                continue;
            }

            let new_indices: Vec<[usize; 3]> = raw
                .par_chunks_exact(3)
                .map(|f| {
                    [
                        vertex_offset + f[0] as usize,
                        vertex_offset + f[1] as usize,
                        vertex_offset + f[2] as usize,
                    ]
                })
                .collect();

            indices.extend(new_indices);
        }
    }

    for child in node.children() {
        process_node(&child, global_transform, buffers, vertices, indices);
    }
}

pub(crate) struct GltfLoader;

impl Loader for GltfLoader {
    fn load(path: &Path) -> Result<Mesh> {
        let (document, buffers, _) = gltf::import(path)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for scene in document.scenes() {
            for node in scene.nodes() {
                process_node(&node, Mat4::IDENTITY, &buffers, &mut vertices, &mut indices);
            }
        }

        if vertices.is_empty() {
            return Err(anyhow!("glTF file contains no geometry"));
        }

        Ok(Mesh { vertices, indices })
    }
}
