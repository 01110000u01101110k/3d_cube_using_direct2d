# 3d Cube

This program is the result of my experiments with 3d math.
I used only direct2D to display the pixels on the screen. 
Also implemented: 
- rotation of the cube can be turned on/off by pressing the x, y and z keys for the corresponding axes. Also, when initializing the cube, you can set whether the cube will rotate around the center of the screen (along the global axes) or around its own center.
- the p key is a pause. That stops the rotation and returns the cube to the state it was in before the rotation started.
- Use the w, a, s, d keys to move the cube about the screen.
- the arrow keys are used to move the camera left, right, forward, backward.
- Use + and - to zoom in and out of the cube.

To test it, try downloading the example, and run using the command `cargo run` for a quick start, or `cargo build --release` for an optimized build, in the latter case, after compilation, look into the target/release project folders, and run the corresponding .exe located in the release folder.

Or edit the file main.js, to play around with the example. You can add new build_cube according to the example, where the Cube structure should be filled in accordingly, this will help you customize each cube you want to display (I hope the structure field names are intuitive). Don't forget to offset the cube using the middle_dot fields that are responsible for the center of the cube. There may also be visual anomalies due to a certain perception of 3D objects by our eyes, for example - if the cubes overlap each other, depth will be lost, this is due to the lack of shadows in this version. As well as the elimination of triangles that do not need to be rendered occurs for each cube separately, so when one cube overlaps another, they do not take into account the depth of each other, but are drawn on top of the one that was drawn first (this will be fixed later, with the addition of a common storage space for all triangles and the separation of the graphics pipeline logic from a specific shape).

An example for drawing multiple cubes:

```rust
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
                    rotate_by_z: false
                },
                deley_rotate_ms: 2.0,
                iner_deley_counter: 0.0,
                rotation_type: RotationTypes::AroundSelf
            },
            builded_cube: BuildedCube::new(),
            to_draw: Vec::new(),
            draw_as_triangles: Vec::new(),
        })
        .build_cube(Cube{
            middle_dot_x: 300.0,
            middle_dot_y: 0.0,
            middle_dot_z: 0.0,
            size: 100.0,
            rotation: Rotatin {
                is_need_rotate: true,
                degree: Degree::new(),
                rotate_directions: RotateDirections {
                    rotate_by_x: false,
                    rotate_by_y: true,
                    rotate_by_z: false
                },
                deley_rotate_ms: 2.0,
                iner_deley_counter: 0.0,
                rotation_type: RotationTypes::AroundSelf
            },
            builded_cube: BuildedCube::new(),
            to_draw: Vec::new(),
            draw_as_triangles: Vec::new(),
        })
        .build_cube(Cube{
            middle_dot_x: -300.0,
            middle_dot_y: 0.0,
            middle_dot_z: 0.0,
            size: 100.0,
            rotation: Rotatin {
                is_need_rotate: true,
                degree: Degree::new(),
                rotate_directions: RotateDirections {
                    rotate_by_x: true,
                    rotate_by_y: false,
                    rotate_by_z: false
                },
                deley_rotate_ms: 2.0,
                iner_deley_counter: 0.0,
                rotation_type: RotationTypes::AroundSelf
            },
            builded_cube: BuildedCube::new(),
            to_draw: Vec::new(),
            draw_as_triangles: Vec::new(),
        })
        .run();
```
