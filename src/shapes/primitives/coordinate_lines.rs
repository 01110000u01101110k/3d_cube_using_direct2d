use windows::{Win32::Graphics::Direct2D::Common::*};
use crate::shapes::primitives::{VectorPoint3D};
use crate::engine_core::{Window};

pub struct CoordinateLines {
    x_coordinate: bool,
    y_coordinate: bool,
    z_coordinate: bool,
}

impl CoordinateLines {
    pub fn new() -> Self {
        Self {
            x_coordinate: true,
            y_coordinate: true,
            z_coordinate: true,
        }
    }

    pub fn draw_coordinate_lines(&self, window: &Window, middle_of_object_x: f32, middle_of_object_y: f32) {
        self.x_coordinate(&window, middle_of_object_x, middle_of_object_y);
        self.y_coordinate(&window, middle_of_object_x, middle_of_object_y);
        self.z_coordinate(&window, middle_of_object_x, middle_of_object_y);
        
    }

    pub fn x_coordinate(&self, window: &Window, middle_of_object_x: f32, middle_of_object_y: f32) {
        let target = window.target.as_ref().unwrap();
        let brush_red = window.brush_red.as_ref().unwrap();

        unsafe {
            target.DrawLine(
                D2D_POINT_2F {
                    x: middle_of_object_x,
                    y: middle_of_object_y,
                },
                D2D_POINT_2F {
                    x: middle_of_object_x + 1000.0,
                    y: middle_of_object_y,
                },
                brush_red,
                4.0,
                &window.style,
            );
        }
    }

    pub fn y_coordinate(&self, window: &Window, middle_of_object_x: f32, middle_of_object_y: f32) {
        let target = window.target.as_ref().unwrap();
        let brush_green = window.brush_green.as_ref().unwrap();

        unsafe {
            target.DrawLine(
                D2D_POINT_2F {
                    x: middle_of_object_x,
                    y: middle_of_object_y,
                },
                D2D_POINT_2F {
                    x: middle_of_object_x,
                    y: middle_of_object_y - 1000.0,
                },
                brush_green,
                4.0,
                &window.style,
            );
        }
    }

    pub fn z_coordinate(&self, window: &Window, middle_of_object_x: f32, middle_of_object_y: f32) {
        let target = window.target.as_ref().unwrap();
        let brush_blue = window.brush_blue.as_ref().unwrap();

        unsafe {
            target.DrawLine(
                D2D_POINT_2F {
                    x: middle_of_object_x,
                    y: middle_of_object_y,
                },
                D2D_POINT_2F {
                    x: middle_of_object_x + 1000.0,
                    y: middle_of_object_y + 1000.0,
                },
                brush_blue,
                4.0,
                &window.style,
            );
        }
    }
}