use std::{
    fs::File,
    io::{self, BufReader},
};

use glam::{Mat4, Quat, Vec3, Vec3A, Vec4};
use obj::{Obj, Vertex, load_obj};
use rayon::prelude::*;

use crate::{MIN_CAM_RADIUS, model::Model, types::Face, types::Pos4};

const SIGMA: f32 = 0.99;

pub fn load(
    file_name: &str,
    scale: Vec3,
    rotation: Quat,
    translation: Vec3,
) -> Result<Object, io::Error> {
    let input = BufReader::new(File::open(file_name)?);
    let model: Obj<Vertex, u32> = load_obj(input).map_err(io::Error::other)?;

    if !model.indices.len().is_multiple_of(3) {
        return Err(io::Error::other(
            "indices are not a multiple of 3. Please triangulate your model.",
        ));
    }

    let size = model.vertices.len();
    let mut vertex = Vec::with_capacity(size);
    let mut normals = Vec::with_capacity(size);

    let Some(max_len_sq) = model
        .vertices
        .par_iter()
        .map(|v| Vec3A::from_array(v.position).length_squared())
        .max_by(|x, y| x.total_cmp(y))
    else {
        return Err(io::Error::other("Couldn't compute the max vertex."));
    };

    let factor = (MIN_CAM_RADIUS * SIGMA) / max_len_sq.sqrt();

    for v in &model.vertices {
        let pos = Vec3A::from_array(v.position) * factor;
        vertex.push(Vec4::new(pos.x, pos.y, pos.z, 1.0));
        normals.push(Vec3A::from_array(v.normal).normalize());
    }

    let faces = model
        .indices
        .chunks_exact(3)
        .map(|face| (face[0] as usize, face[1] as usize, face[2] as usize))
        .collect();

    Ok(Object::new(
        model,
        vertex,
        faces,
        normals,
        scale,
        rotation,
        translation,
    ))
}

pub(crate) struct Object {
    _model_matrix: Mat4,
    vertex: Vec<Vec4>,
    _normals: Vec<Vec3A>,
    faces: Vec<Face>,
    _model: Obj<Vertex, u32>,
}

impl Object {
    pub fn new(
        model: Obj<Vertex, u32>,
        vertex: Vec<Pos4>,
        faces: Vec<Face>,
        normals: Vec<Vec3A>,
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Self {
        let model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        let mut transformed_vertex = Vec::with_capacity(vertex.len());
        for vertex in vertex.iter() {
            transformed_vertex.push(model_matrix * *vertex);
        }

        Object {
            _model: model,
            vertex: transformed_vertex,
            faces,
            _normals: normals,
            _model_matrix: model_matrix,
        }
    }
}

impl Model for Object {
    fn get_points(&self) -> &[Pos4] {
        &self.vertex
    }

    fn get_triangles(&self) -> &[Face] {
        &self.faces
    }
}
