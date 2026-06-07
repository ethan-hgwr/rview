use anyhow::Result;
use glam::{Mat3A, Mat4, Vec3A, Vec4};
use rayon::prelude::*;
use std::io::stdout;

use crate::{
    camera::{Camera, CameraState},
    framebuffer::{Buffer, Framebuffer},
    loaders::Mesh,
    palette::map_brightness_to_char,
    preprocessors::Preprocessor,
    raster::fill_triangle,
};

pub(crate) struct Pipeline<P> {
    projection_matrix: Mat4,
    meshes: Vec<GpuMesh>,
    framebuffer: Framebuffer<P>,
    camera: Camera,
}

/// Render-ready mesh
struct GpuMesh {
    vertices: Vec<Vec4>,
    face_normals: Vec<Vec3A>,
    indices: Vec<[usize; 3]>,
    clip_verts: Vec<Vec4>,
    view_normals: Vec<Vec3A>,
}

impl GpuMesh {
    fn from_mesh(mesh: Mesh) -> Self {
        let vertex_count = mesh.vertices.len();
        let face_count = mesh.indices.len();

        let vertices: Vec<Vec4> = mesh
            .vertices
            .par_iter()
            .map(|v| Vec4::new(v[0], v[1], v[2], 1.0))
            .collect();

        let face_normals: Vec<Vec3A> = mesh
            .indices
            .par_iter()
            .map(|&[i0, i1, i2]| {
                let a = Vec3A::new(
                    mesh.vertices[i0][0],
                    mesh.vertices[i0][1],
                    mesh.vertices[i0][2],
                );
                let b = Vec3A::new(
                    mesh.vertices[i1][0],
                    mesh.vertices[i1][1],
                    mesh.vertices[i1][2],
                );
                let c = Vec3A::new(
                    mesh.vertices[i2][0],
                    mesh.vertices[i2][1],
                    mesh.vertices[i2][2],
                );
                (b - a).cross(c - a).normalize()
            })
            .collect();

        Self {
            vertices,
            face_normals,
            indices: mesh.indices,
            clip_verts: Vec::with_capacity(vertex_count),
            view_normals: Vec::with_capacity(face_count),
        }
    }
}

impl<T> Pipeline<T>
where
    T: Copy,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
        meshes: Vec<Mesh>,
        framebuffer: Framebuffer<T>,
        preprocessors: &[Box<dyn Preprocessor>],
        camera: Camera,
    ) -> Result<Self> {
        let gpu_meshes = meshes
            .into_iter()
            .map(|mut mesh| {
                for p in preprocessors {
                    p.process(&mut mesh)?
                }
                Ok(GpuMesh::from_mesh(mesh))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Pipeline {
            projection_matrix: Mat4::perspective_rh(fov, aspect_ratio, near, far),
            meshes: gpu_meshes,
            framebuffer,
            camera,
        })
    }
}

impl Pipeline<char> {
    pub fn apply_camera_state(&mut self, state: &CameraState) {
        self.camera.update_radius(state.radius());
        self.camera.rotate_x(state.pitch());
        self.camera.rotate_y(state.yaw());
    }

    pub fn render(&mut self) -> Result<()> {
        self.framebuffer.clear();

        let view_matrix = *self.camera.get_view_matrix();
        let view_proj = self.projection_matrix * view_matrix;
        let normal_matrix = Mat3A::from_mat4(view_matrix);
        let light_dir = (normal_matrix * self.camera.get_position().normalize()).normalize();

        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

        for mesh in &mut self.meshes {
            mesh.clip_verts.clear();
            mesh.view_normals.clear();

            mesh.clip_verts
                .par_extend(mesh.vertices.par_iter().map(|&v| view_proj * v));

            mesh.view_normals.par_extend(
                mesh.face_normals
                    .par_iter()
                    .map(|&n| (normal_matrix * n).normalize()),
            );

            for (tri_idx, &[i0, i1, i2]) in mesh.indices.iter().enumerate() {
                let normal = mesh.view_normals[tri_idx];

                if normal.z < 0.0 {
                    continue;
                }

                let brightness = normal.dot(light_dir).clamp(0.0, 1.0);
                let shade = map_brightness_to_char(brightness);

                let pa = project_to_screen(mesh.clip_verts[i0], width, height);
                let pb = project_to_screen(mesh.clip_verts[i1], width, height);
                let pc = project_to_screen(mesh.clip_verts[i2], width, height);

                fill_triangle(&mut self.framebuffer, pa, pb, pc, shade);
            }
        }

        self.framebuffer.write_io(&mut stdout())
    }
}

fn project_to_screen(point: Vec4, width: usize, height: usize) -> Vec3A {
    let ndc = point.truncate() / (point.w + f32::EPSILON);
    Vec3A::new(
        (ndc.x + 1.0) * 0.5 * width as f32,
        (1.0 - ndc.y) * 0.5 * height as f32,
        ndc.z,
    )
}
