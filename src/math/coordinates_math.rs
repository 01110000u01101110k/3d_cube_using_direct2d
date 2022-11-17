use crate::shapes::{VectorPoint3D};

pub fn from_cartesian_to_screen_coordinates(
    vector: &VectorPoint3D, 
    screen_width: f32, 
    screen_height: f32
) -> VectorPoint3D {
    let midle_of_coordinate_width = screen_width / 2.0;
    let midle_of_coordinate_height = screen_height / 2.0;

    let mut result_vector = VectorPoint3D::new();

    result_vector.x = midle_of_coordinate_width + vector.x;
    result_vector.y = midle_of_coordinate_height - vector.y;

    result_vector
}