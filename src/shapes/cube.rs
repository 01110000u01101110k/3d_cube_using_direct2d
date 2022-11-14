use windows::{core::*};

use crate::engine_core::{Window};
use crate::shapes::primitives::{VectorPoint3D, VectorPoint2D};
use crate::shapes::{Shape};
use crate::math::{transoformate_3d_vector_to_2d, from_cartesian_to_screen_coordinates};

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

impl Shape for Cube {
    fn new() -> Self {
        Self {
            middle_dot_x: 0.0,
            middle_dot_y: 0.0,
            middle_dot_z: 0.0,
            size: 0.0
        }
    }

    fn build_shape(&self, window: Window) {
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

        cube[0].x = self.middle_dot_x - (self.size / 2.0);
        cube[0].y = self.middle_dot_y + (self.size / 2.0);
        cube[0].z = self.middle_dot_z + (self.size / 2.0);

        cube[1].x = self.middle_dot_x + (self.size / 2.0);
        cube[1].y = self.middle_dot_y + (self.size / 2.0);
        cube[1].z = self.middle_dot_z + (self.size / 2.0);

        cube[2].x = self.middle_dot_x - (self.size / 2.0);
        cube[2].y = self.middle_dot_y - (self.size / 2.0);
        cube[2].z = self.middle_dot_z - (self.size / 2.0);

        cube[3].x = self.middle_dot_x + (self.size / 2.0);
        cube[3].y = self.middle_dot_y - (self.size / 2.0);
        cube[3].z = self.middle_dot_z - (self.size / 2.0);

        cube[4].x = self.middle_dot_x - (self.size / 2.0);
        cube[4].y = self.middle_dot_y + (self.size / 2.0);
        cube[4].z = self.middle_dot_z + (self.size / 2.0);

        cube[5].x = self.middle_dot_x + (self.size / 2.0);
        cube[5].y = self.middle_dot_y + (self.size / 2.0);
        cube[5].z = self.middle_dot_z + (self.size / 2.0);

        cube[6].x = self.middle_dot_x - (self.size / 2.0);
        cube[6].y = self.middle_dot_y - (self.size / 2.0);
        cube[6].z = self.middle_dot_z - (self.size / 2.0);

        cube[7].x = self.middle_dot_x + (self.size / 2.0);
        cube[7].y = self.middle_dot_y - (self.size / 2.0);
        cube[7].z = self.middle_dot_z - (self.size / 2.0);

        let mut cube_2d_proection_on_screen: Vec<VectorPoint2D> = Vec::new();

        cube.iter().for_each(|vector| {
            let vector_2d_proection_on_screen = transoformate_3d_vector_to_2d(vector);
            let result_vector = from_cartesian_to_screen_coordinates(vector_2d_proection_on_screen, 1920.0, 1080.0);
            cube_2d_proection_on_screen.push(result_vector);
        });

        /*target.DrawLine(
            D2D_POINT_2F {
                x: self.,
                y: ,
            },
            D2D_POINT_2F {
                x: ,
                y: ,
            },
            brush,
            0.0,
            &self.style,
        );*/
    }
}