use crate::preprocessors::Preprocessor;
use anyhow::{Result, anyhow};
use glam::Vec3A;
use rayon::prelude::*;

pub(crate) struct CenterModel;

impl Preprocessor for CenterModel {
    fn process(&self, mesh: &mut crate::loaders::Mesh) -> Result<()> {
        let (weighted_sum, total_area) = mesh
            .indices
            .par_iter()
            .map(|&[i0, i1, i2]| {
                let a = Vec3A::from_array(mesh.vertices[i0]);
                let b = Vec3A::from_array(mesh.vertices[i1]);
                let c = Vec3A::from_array(mesh.vertices[i2]);
                let area = (b - a).cross(c - a).length() * 0.5;
                ((a + b + c) / 3.0 * area, area)
            })
            .reduce(|| (Vec3A::ZERO, 0.0), |x, y| (x.0 + y.0, x.1 + y.1));

        if total_area <= f32::EPSILON {
            return Err(anyhow!("Mesh has no surface area."));
        }
        let center = weighted_sum / total_area;

        mesh.vertices.par_iter_mut().for_each(|v| {
            *v = (Vec3A::from_array(*v) - center).to_array();
        });

        Ok(())
    }
}
