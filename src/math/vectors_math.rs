use crate::shapes::{VectorPoint3D};

pub fn perspective_projection(
    vector_3d: &VectorPoint3D,
    screen_width: f32, 
    screen_height: f32
) -> VectorPoint3D {
    let mut new_vector = VectorPoint3D::new();

    let deph = screen_width / (screen_width + vector_3d.z);

    new_vector.x = vector_3d.x * deph;
    new_vector.y = vector_3d.y * deph;

    new_vector
}