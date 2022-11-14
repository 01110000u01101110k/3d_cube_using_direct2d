use spinning_cube::engine_core::{Engine};
use spinning_cube::shapes::{Cube};

fn main() {
    Engine::new()
    .build_cube(Cube{
        middle_dot_x: 30.0,
        middle_dot_y: 30.0,
        middle_dot_z: 30.0,
        size: 15.0
    })
    .run();
}