use crate::engine_core::{BASE_3D_DEPH};
use crate::shapes::{VectorPoint3D};

pub fn transoformate_3d_vector_to_2d_screen_vector(
    vector_3d: &VectorPoint3D,
) -> VectorPoint3D {
    let mut new_vector = VectorPoint3D::new();

    let deph = BASE_3D_DEPH / (BASE_3D_DEPH + vector_3d.z);

    new_vector.x = vector_3d.x * deph;
    new_vector.y = vector_3d.y * deph;

    new_vector
}