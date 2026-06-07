use crate::constants::*;
use crate::preprocessors::Preprocessor;
use anyhow::Result;
use glam::Vec3A;
use rayon::prelude::*;

const SIGMA: f32 = 0.99;

/// Normalizes the model to make it fit in the `MIN_CAM_RADIUS`
/// to prevent the camera from going into the model.
pub(crate) struct CamNormalizer;

impl Preprocessor for CamNormalizer {
    fn process(&self, mesh: &mut crate::loaders::Mesh) -> Result<()> {
        let Some(max_len_sq) = mesh
            .vertices
            .par_iter()
            .map(|v| Vec3A::from_array(*v).length_squared())
            .max_by(|x, y| x.total_cmp(y))
        else {
            return Err(anyhow::anyhow!("Mesh has no vertices."));
        };

        let factor = (MIN_CAM_RADIUS * SIGMA) / max_len_sq.sqrt();

        mesh.vertices.par_iter_mut().for_each(|v| {
            let scaled = Vec3A::from_array(*v) * factor;
            *v = scaled.to_array();
        });

        Ok(())
    }
}
