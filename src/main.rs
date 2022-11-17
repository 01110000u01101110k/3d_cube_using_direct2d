use spinning_cube::engine_core::{Engine};
use spinning_cube::shapes::{Cube, BuildedCube};
use spinning_cube::math::{
    Degree,
    Rotatin,
    RotateDirections
};

fn main() {
    Engine::new()
    .build_cube(Cube{
        middle_dot_x: 0.0,
        middle_dot_y: 0.0,
        middle_dot_z: 0.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: 100.0,
        middle_dot_y: 0.0,
        middle_dot_z: 0.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: -100.0,
        middle_dot_y: 0.0,
        middle_dot_z: 0.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: 0.0,
        middle_dot_y: 0.0,
        middle_dot_z: 100.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: 0.0,
        middle_dot_y: 0.0,
        middle_dot_z: -100.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: -100.0,
        middle_dot_y: 0.0,
        middle_dot_z: -100.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: 100.0,
        middle_dot_y: 0.0,
        middle_dot_z: -100.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: -100.0,
        middle_dot_y: 0.0,
        middle_dot_z: 100.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })

    .build_cube(Cube{
        middle_dot_x: 100.0,
        middle_dot_y: 0.0,
        middle_dot_z: 100.0,
        size: 100.0,
        rotation: Rotatin {
            is_need_rotate: true,
            degree: Degree::new(),
            rotate_directions: RotateDirections {
                rotate_by_x: true,
                rotate_by_y: true,
                rotate_by_z: true
            },
            deley_rotate_ms: 2.0,
            iner_deley_counter: 0.0,
        },
        builded_cube: BuildedCube::new(),
        to_draw: Vec::new()
    })
    .run();
}