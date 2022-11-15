use spinning_cube::engine_core::{Engine};
use spinning_cube::shapes::{Cube};

fn main() {
    Engine::new()
    .build_cube(Cube{
        middle_dot_x: 400.0,
        middle_dot_y: 400.0,
        middle_dot_z: 100.0,
        size: 30.0
    })
    .run();
}