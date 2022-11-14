use crate::shapes::{VectorPoint2D};

pub fn from_cartesian_to_screen_coordinates(
    vector: VectorPoint2D, 
    screen_width: f32, 
    screen_height: f32
) -> VectorPoint2D {
    let midle_of_coordinate_width = screen_width / 2.0;
    let midle_of_coordinate_height = screen_height / 2.0;

    let mut result_vector_2d = VectorPoint2D::new();

    result_vector_2d.x = midle_of_coordinate_width + vector.x;
    result_vector_2d.y = midle_of_coordinate_height - vector.y;

    result_vector_2d
}