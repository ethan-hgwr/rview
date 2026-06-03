use crate::{types::Face, types::Pos4};

pub(crate) trait Model {
    fn get_points(&self) -> &[Pos4];
    fn get_triangles(&self) -> &[Face];
}
