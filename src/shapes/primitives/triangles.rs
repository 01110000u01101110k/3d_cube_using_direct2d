use windows::{Win32::Graphics::Direct2D::Common::*};
use crate::shapes::primitives::{VectorPoint3D};
use crate::engine_core::{Window};

#[derive(Debug, Clone)]
pub struct Triangles {
    point_1: VectorPoint3D,
    point_2: VectorPoint3D,
    point_3: VectorPoint3D
}

impl Triangles {
    pub fn new() -> Self {
        Self {
            point_1: VectorPoint3D::new(),
            point_2: VectorPoint3D::new(),
            point_3: VectorPoint3D::new(),
        }
    }

    pub fn set_triangle(&mut self, point_1: VectorPoint3D, point_2: VectorPoint3D,  point_3: VectorPoint3D) -> Self {
        self.point_1 = point_1;
        self.point_2 = point_2;
        self.point_3 = point_3;

        self.clone()
    }

    pub fn draw_triangle(&self, window: &Window) {
        let target = window.target.as_ref().unwrap();
        let brush = window.brush.as_ref().unwrap();

        unsafe {
            target.DrawLine(
                D2D_POINT_2F {
                    x: self.point_1.x,
                    y: self.point_1.y,
                },
                D2D_POINT_2F {
                    x: self.point_2.x,
                    y: self.point_2.y,
                },
                brush,
                4.0,
                &window.style,
            );
    
            target.DrawLine(
                D2D_POINT_2F {
                    x: self.point_2.x,
                    y: self.point_2.y,
                },
                D2D_POINT_2F {
                    x: self.point_3.x,
                    y: self.point_3.y,
                },
                brush,
                4.0,
                &window.style,
            );
    
            target.DrawLine(
                D2D_POINT_2F {
                    x: self.point_3.x,
                    y: self.point_3.y,
                },
                D2D_POINT_2F {
                    x: self.point_1.x,
                    y: self.point_1.y,
                },
                brush,
                4.0,
                &window.style,
            );
        }
    }
}