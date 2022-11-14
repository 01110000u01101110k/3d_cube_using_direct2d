use crate::engine_core::{BASE_3D_DEPH};
use crate::shapes::{VectorPoint3D, VectorPoint2D};

pub fn transoformate_3d_vector_to_2d(
    vector_3d: &VectorPoint3D
) -> VectorPoint2D {
    let mut new_2d_vector = VectorPoint2D::new();

    let deph = BASE_3D_DEPH / (BASE_3D_DEPH + vector_3d.z);

    new_2d_vector.x *= deph;
    new_2d_vector.x *= deph;

    new_2d_vector
}