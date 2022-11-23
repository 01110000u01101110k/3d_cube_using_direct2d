use windows::{Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*, Win32::UI::WindowsAndMessaging::*};

use crate::engine_core::{Window};
use crate::shapes::{CoordinateLines};
use crate::shapes::primitives::{VectorPoint3D};
use crate::math::{
    transoformate_3d_vector_to_2d_screen, 
    from_cartesian_to_screen_coordinates,
    Rotatin,
    RotationTypes
};

#[derive(Clone, Debug)]
pub struct BuildedCube {
    pub points: Vec<VectorPoint3D>,
    pub is_builded: bool
}

impl BuildedCube {
    pub fn new() -> Self {
        let points: Vec<VectorPoint3D> = vec![
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
        ];

        Self {
            points,
            is_builded: false
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    pub middle_dot_x: f32,
    pub middle_dot_y: f32,
    pub middle_dot_z: f32,
    pub size: f32,
    pub rotation: Rotatin,
    pub builded_cube: BuildedCube,
    pub to_draw: Vec<VectorPoint3D>,
}

impl Cube {
    pub fn set_cube_info(&self, new_cube: Cube) -> Self {
        Self {
            middle_dot_x: new_cube.middle_dot_x,
            middle_dot_y: new_cube.middle_dot_y,
            middle_dot_z: new_cube.middle_dot_z,
            size: new_cube.size,
            rotation: new_cube.rotation,
            builded_cube: new_cube.builded_cube,
            to_draw: new_cube.to_draw
        }
    }
}

impl Cube {
    pub fn new() -> Self {
        Self {
            middle_dot_x: 0.0,
            middle_dot_y: 0.0,
            middle_dot_z: 0.0,
            size: 0.0,
            rotation: Rotatin::new(),
            builded_cube: BuildedCube::new(),
            to_draw: Vec::new()
        }
    }

    pub fn try_to_rotate_shape(&mut self, shape: &mut BuildedCube) {
        if self.rotation.is_need_rotate == true {
            self.rotation.iner_deley_counter += 1.0;
            
            if self.rotation.iner_deley_counter == self.rotation.deley_rotate_ms {

                self.rotation.iner_deley_counter = 0.0;

                let roatate_degree = 0.05;

                if self.rotation.rotate_directions.rotate_by_x {
                    self.rotation.rotation_by_x(roatate_degree);
                }
                if self.rotation.rotate_directions.rotate_by_y {
                    self.rotation.rotation_by_y(roatate_degree);
                }
                if self.rotation.rotate_directions.rotate_by_z {
                    self.rotation.rotation_by_z(roatate_degree);
                }
                
                self.rotation.rotate_shape(shape);
            } else {
                self.rotation.rotate_shape(shape);
            }
        }
    }

    pub fn build_shape(&mut self, hwnd: HWND) {
        if !self.builded_cube.is_builded {
            match self.rotation.rotation_type {
                RotationTypes::AroundSelf => {
                    self.build_cube_from_middle_points(0.0, 0.0, 0.0);
                },
                RotationTypes::AroundGlobalCoordinates => {
                    self.build_cube_from_middle_points(self.middle_dot_x, self.middle_dot_y, self.middle_dot_z);
                },
                RotationTypes::None => ()
            }
        }

        let mut builded_cube = self.builded_cube.clone();
        let mut cube_2d_proection_on_screen: Vec<VectorPoint3D> = Vec::new();

        let mut client_window_size = RECT::default();
        unsafe {
            GetClientRect(hwnd, &mut client_window_size);
        }

        self.try_to_rotate_shape(&mut builded_cube);

        builded_cube.points.iter_mut().for_each(|vector| {

            match self.rotation.rotation_type {
                RotationTypes::AroundSelf => {
                    vector.x += self.middle_dot_x;
                    vector.y += self.middle_dot_y;
                    vector.z += self.middle_dot_z;
                },
                _ => ()
            }

            let vector_2d_proection_on_screen = transoformate_3d_vector_to_2d_screen(
                vector,
                client_window_size.right as f32,
                client_window_size.bottom as f32
            );

            let result_vector = from_cartesian_to_screen_coordinates(
                &vector_2d_proection_on_screen, 
                client_window_size.right as f32,
                client_window_size.bottom as f32
            );

            cube_2d_proection_on_screen.push(result_vector);
        });

        self.to_draw = cube_2d_proection_on_screen;
    }

    fn build_cube_from_middle_points(&mut self, middle_dot_x: f32, middle_dot_y: f32, middle_dot_z: f32) -> &mut Self {
        self.builded_cube.points[0].x = middle_dot_x - (self.size / 2.0);
        self.builded_cube.points[0].y = middle_dot_y + (self.size / 2.0);
        self.builded_cube.points[0].z = middle_dot_z - (self.size / 2.0);

        self.builded_cube.points[1].x = middle_dot_x + (self.size / 2.0);
        self.builded_cube.points[1].y = middle_dot_y + (self.size / 2.0);
        self.builded_cube.points[1].z = middle_dot_z - (self.size / 2.0);

        self.builded_cube.points[2].x = middle_dot_x - (self.size / 2.0);
        self.builded_cube.points[2].y = middle_dot_y - (self.size / 2.0);
        self.builded_cube.points[2].z = middle_dot_z - (self.size / 2.0);

        self.builded_cube.points[3].x = middle_dot_x + (self.size / 2.0);
        self.builded_cube.points[3].y = middle_dot_y - (self.size / 2.0);
        self.builded_cube.points[3].z = middle_dot_z - (self.size / 2.0);

        self.builded_cube.points[4].x = middle_dot_x - (self.size / 2.0);
        self.builded_cube.points[4].y = middle_dot_y + (self.size / 2.0);
        self.builded_cube.points[4].z = middle_dot_z + (self.size / 2.0);

        self.builded_cube.points[5].x = middle_dot_x + (self.size / 2.0);
        self.builded_cube.points[5].y = middle_dot_y + (self.size / 2.0);
        self.builded_cube.points[5].z = middle_dot_z + (self.size / 2.0);

        self.builded_cube.points[6].x = middle_dot_x - (self.size / 2.0);
        self.builded_cube.points[6].y = middle_dot_y - (self.size / 2.0);
        self.builded_cube.points[6].z = middle_dot_z + (self.size / 2.0);

        self.builded_cube.points[7].x = middle_dot_x + (self.size / 2.0);
        self.builded_cube.points[7].y = middle_dot_y - (self.size / 2.0);
        self.builded_cube.points[7].z = middle_dot_z + (self.size / 2.0);

        self.builded_cube.is_builded = true;

        self
    }

    pub fn draw_cube(&self, window: &Window) {
        let target = window.target.as_ref().unwrap();
        let brush = window.brush.as_ref().unwrap();

        /*
        які точки повинні бути зєднані:
        
        p_0 -> p_1 -> p_3 -> p_2 -> p_0

        p_4 -> p_5 -> p_7 -> p_6 -> p_4

        p_0 -> p_4

        p_1 -> p_5

        p_2 -> p_6

        p_3 -> p_7
        */

        unsafe{
            // 1
            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[0].x,
                    y: self.to_draw[0].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[1].x,
                    y: self.to_draw[1].y,
                },
                brush,
                4.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[1].x,
                    y: self.to_draw[1].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[3].x,
                    y: self.to_draw[3].y,
                },
                brush,
                4.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[3].x,
                    y: self.to_draw[3].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[2].x,
                    y: self.to_draw[2].y,
                },
                brush,
                4.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[2].x,
                    y: self.to_draw[2].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[0].x,
                    y: self.to_draw[0].y,
                },
                brush,
                4.0,
                &window.style,
            );

            // 2

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[4].x,
                    y: self.to_draw[4].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[5].x,
                    y: self.to_draw[5].y,
                },
                brush,
                4.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[5].x,
                    y: self.to_draw[5].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[7].x,
                    y: self.to_draw[7].y,
                },
                brush,
                4.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[7].x,
                    y: self.to_draw[7].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[6].x,
                    y: self.to_draw[6].y,
                },
                brush,
                4.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[6].x,
                    y: self.to_draw[6].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[4].x,
                    y: self.to_draw[4].y,
                },
                brush,
                4.0,
                &window.style,
            );

            // 3

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[0].x,
                    y: self.to_draw[0].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[4].x,
                    y: self.to_draw[4].y,
                },
                brush,
                4.0,
                &window.style,
            );

            // 4

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[1].x,
                    y: self.to_draw[1].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[5].x,
                    y: self.to_draw[5].y,
                },
                brush,
                4.0,
                &window.style,
            );

            // 5

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[2].x,
                    y: self.to_draw[2].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[6].x,
                    y: self.to_draw[6].y,
                },
                brush,
                4.0,
                &window.style,
            );

            // 6

            target.DrawLine(
                D2D_POINT_2F {
                    x: self.to_draw[3].x,
                    y: self.to_draw[3].y,
                },
                D2D_POINT_2F {
                    x: self.to_draw[7].x,
                    y: self.to_draw[7].y,
                },
                brush,
                4.0,
                &window.style,
            );
        }            


        // draw coordinate lines

        //CoordinateLines::new().draw_coordinate_lines(&window, self.to_draw[0].x + (self.to_draw[0].x / 2.0), self.to_draw[0].y + (self.to_draw[0].y / 2.0));
    }
}