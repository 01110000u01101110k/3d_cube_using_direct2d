use windows::{core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*, Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*, Win32::System::Com::*, Win32::System::LibraryLoader::*, Win32::System::Performance::*, Win32::System::SystemInformation::GetLocalTime, Win32::UI::Animation::*, Win32::UI::WindowsAndMessaging::*};

use crate::engine_core::{Window};
use crate::shapes::primitives::{VectorPoint3D, VectorPoint2D};
use crate::shapes::{Shape};
use crate::math::{transoformate_3d_vector_to_2d, from_cartesian_to_screen_coordinates};

#[derive(Clone, Debug)]
pub struct Cube {
    pub middle_dot_x: f32,
    pub middle_dot_y: f32,
    pub middle_dot_z: f32,
    pub size: f32
}

impl Cube {
    pub fn set_cube_info(&self, new_cube: Cube) -> Self {
        Self {
            middle_dot_x: new_cube.middle_dot_x,
            middle_dot_y: new_cube.middle_dot_y,
            middle_dot_z: new_cube.middle_dot_z,
            size: new_cube.size,
        }
    }
}

impl Cube {
    pub fn new() -> Self {
        Self {
            middle_dot_x: 0.0,
            middle_dot_y: 0.0,
            middle_dot_z: 0.0,
            size: 0.0
        }
    }

    pub fn build_shape(&self, window: &Window) {
        let target = window.target.as_ref().unwrap();
        let brush = window.brush.as_ref().unwrap();

        let mut cube: Vec<VectorPoint3D> = vec![
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
            VectorPoint3D::new(),
        ];

        cube[0].x = self.middle_dot_x - (self.size * 7.0);
        cube[0].y = self.middle_dot_y + (self.size * 7.0);
        cube[0].z = self.middle_dot_z - self.size;

        cube[1].x = self.middle_dot_x + (self.size * 7.0);
        cube[1].y = self.middle_dot_y + (self.size * 7.0);
        cube[1].z = self.middle_dot_z - self.size;

        cube[2].x = self.middle_dot_x - (self.size * 7.0);
        cube[2].y = self.middle_dot_y - (self.size * 7.0);
        cube[2].z = self.middle_dot_z - self.size;

        cube[3].x = self.middle_dot_x + (self.size * 7.0);
        cube[3].y = self.middle_dot_y - (self.size * 7.0);
        cube[3].z = self.middle_dot_z - self.size;

        cube[4].x = self.middle_dot_x - (self.size * 7.0);
        cube[4].y = self.middle_dot_y + (self.size * 7.0);
        cube[4].z = self.middle_dot_z + self.size;

        cube[5].x = self.middle_dot_x + (self.size * 7.0);
        cube[5].y = self.middle_dot_y + (self.size * 7.0);
        cube[5].z = self.middle_dot_z + self.size;

        cube[6].x = self.middle_dot_x - (self.size * 7.0);
        cube[6].y = self.middle_dot_y - (self.size * 7.0);
        cube[6].z = self.middle_dot_z + self.size;

        cube[7].x = self.middle_dot_x + (self.size * 7.0);
        cube[7].y = self.middle_dot_y - (self.size * 7.0);
        cube[7].z = self.middle_dot_z + self.size;

        let mut cube_2d_proection_on_screen: Vec<VectorPoint2D> = Vec::new();

        cube.iter().for_each(|vector| {
            let vector_2d_proection_on_screen = transoformate_3d_vector_to_2d(vector);

            let mut result_vector = from_cartesian_to_screen_coordinates(
                vector_2d_proection_on_screen, 
                1920.0, 
                1080.0
            );
            
            cube_2d_proection_on_screen.push(result_vector);
        });

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
                    x: cube_2d_proection_on_screen[0].x,
                    y: cube_2d_proection_on_screen[0].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[1].x,
                    y: cube_2d_proection_on_screen[1].y,
                },
                brush,
                1.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[1].x,
                    y: cube_2d_proection_on_screen[1].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[3].x,
                    y: cube_2d_proection_on_screen[3].y,
                },
                brush,
                1.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[3].x,
                    y: cube_2d_proection_on_screen[3].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[2].x,
                    y: cube_2d_proection_on_screen[2].y,
                },
                brush,
                1.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[2].x,
                    y: cube_2d_proection_on_screen[2].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[0].x,
                    y: cube_2d_proection_on_screen[0].y,
                },
                brush,
                1.0,
                &window.style,
            );

            // 2

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[4].x,
                    y: cube_2d_proection_on_screen[4].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[5].x,
                    y: cube_2d_proection_on_screen[5].y,
                },
                brush,
                1.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[5].x,
                    y: cube_2d_proection_on_screen[5].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[7].x,
                    y: cube_2d_proection_on_screen[7].y,
                },
                brush,
                1.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[7].x,
                    y: cube_2d_proection_on_screen[7].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[6].x,
                    y: cube_2d_proection_on_screen[6].y,
                },
                brush,
                1.0,
                &window.style,
            );

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[6].x,
                    y: cube_2d_proection_on_screen[6].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[4].x,
                    y: cube_2d_proection_on_screen[4].y,
                },
                brush,
                1.0,
                &window.style,
            );

            // 3

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[0].x,
                    y: cube_2d_proection_on_screen[0].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[4].x,
                    y: cube_2d_proection_on_screen[4].y,
                },
                brush,
                1.0,
                &window.style,
            );

            // 4

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[1].x,
                    y: cube_2d_proection_on_screen[1].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[5].x,
                    y: cube_2d_proection_on_screen[5].y,
                },
                brush,
                1.0,
                &window.style,
            );

            // 5

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[2].x,
                    y: cube_2d_proection_on_screen[2].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[6].x,
                    y: cube_2d_proection_on_screen[6].y,
                },
                brush,
                1.0,
                &window.style,
            );

            // 6

            target.DrawLine(
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[3].x,
                    y: cube_2d_proection_on_screen[3].y,
                },
                D2D_POINT_2F {
                    x: cube_2d_proection_on_screen[7].x,
                    y: cube_2d_proection_on_screen[7].y,
                },
                brush,
                1.0,
                &window.style,
            );
        }
    }
}