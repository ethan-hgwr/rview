use crate::loaders::Mesh;
use anyhow::Result;

pub(crate) mod cam_normalizer;
pub(crate) mod center_model;

pub(crate) trait Preprocessor {
    fn process(&self, mesh: &mut Mesh) -> Result<()>;
}
