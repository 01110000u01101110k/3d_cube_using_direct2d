use windows::Win32::Graphics::Direct2D::{Common::*, ID2D1SolidColorBrush};
use crate::shapes::primitives::{VectorPoint3D};
use crate::engine_core::{Window};

#[derive(Debug, Clone)]
pub struct Triangle {
    point_1: VectorPoint3D,
    point_2: VectorPoint3D,
    point_3: VectorPoint3D,
    pub is_visible: bool
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            point_1: VectorPoint3D::new(),
            point_2: VectorPoint3D::new(),
            point_3: VectorPoint3D::new(),
            is_visible: false
        }
    }

    pub fn set_triangle(&mut self, point_1: VectorPoint3D, point_2: VectorPoint3D,  point_3: VectorPoint3D) -> Self {
        self.point_1 = point_1;
        self.point_2 = point_2;
        self.point_3 = point_3;

        self.clone()
    }

    pub fn find_perpendicular_direction_to_triangle(&self) -> f32 { 
        (self.point_2.x - self.point_1.x) * (self.point_3.y - self.point_1.y) - 
        (self.point_2.y - self.point_1.y) * (self.point_3.x - self.point_1.x)
    }

    pub fn fill_triangle_color(&self, window: &Window, brush: &ID2D1SolidColorBrush) {
        /*
            Реалізую алгоритм який дозволяє заповнити трикутник пікселями (растеризувати), 
            Це працює за рахунок "плоских" (у верхній чи ніжній частині) трикутників.
            У випадку не "плоского" трикутник, трикутник ділиться на 2 плоских трикутника.
        */

        fn sort_triangle_points(triangle: &Triangle) -> Triangle {
            let mut sorted_triangle: Triangle = triangle.clone();
            let mut swaped_point: VectorPoint3D;

            // sort step 1
            if sorted_triangle.point_1.y > sorted_triangle.point_2.y {
                swaped_point = sorted_triangle.point_1.clone();

                sorted_triangle.point_1 = sorted_triangle.point_2.clone();
                sorted_triangle.point_2 = swaped_point;
            } 
            // sort step 2
            if sorted_triangle.point_1.y > sorted_triangle.point_3.y {
                swaped_point = sorted_triangle.point_1.clone();

                sorted_triangle.point_1 = sorted_triangle.point_3.clone();
                sorted_triangle.point_3 = swaped_point;
            }
            // sort step 3
            if sorted_triangle.point_2.y > sorted_triangle.point_3.y {
                swaped_point = sorted_triangle.point_2.clone();

                sorted_triangle.point_2 = sorted_triangle.point_3.clone();
                sorted_triangle.point_3 = swaped_point;
            }

            sorted_triangle
        }

        fn fill_bottom_flat_triangle(window: &Window, sorted_triangle: Triangle, brush: &ID2D1SolidColorBrush) {
            let target = window.target.as_ref().unwrap();

            let mut index_y = sorted_triangle.point_1.y;
            let mut first_target_point_x = sorted_triangle.point_1.x;
            let mut second_target_point_x = sorted_triangle.point_1.x;

            let first_length = (sorted_triangle.point_2.x - sorted_triangle.point_1.x) / (sorted_triangle.point_2.y - sorted_triangle.point_1.y);
            let second_length = (sorted_triangle.point_3.x - sorted_triangle.point_1.x) / (sorted_triangle.point_3.y - sorted_triangle.point_1.y);
            
            while index_y < sorted_triangle.point_2.y {
                unsafe {
                    target.DrawLine(
                        D2D_POINT_2F {
                            x: first_target_point_x,
                            y: index_y,
                        },
                        D2D_POINT_2F {
                            x: second_target_point_x,
                            y: index_y,
                        },
                        brush,
                        2.0,
                        &window.style,
                    );
                }

                /*let mut target_point = first_target_point_x;

                while target_point < second_target_point_x {
                    let rect = D2D_RECT_F {
                        left: target_point,
                        top: index_y,
                        right: target_point + 1.0,
                        bottom: index_y + 1.0,
                    };
                    unsafe {
                        target.FillRectangle(&rect, brush);
                    };

                    target_point += 1.0;
                }*/

                first_target_point_x += first_length;
                second_target_point_x += second_length;

                index_y += 1.0;
            }
        }

        fn fill_top_flat_triangle(window: &Window, sorted_triangle: Triangle, brush: &ID2D1SolidColorBrush) {
            let target = window.target.as_ref().unwrap();

            let mut index_y = sorted_triangle.point_3.y;
            let mut first_target_point_x = sorted_triangle.point_3.x;
            let mut second_target_point_x = sorted_triangle.point_3.x;

            let first_length = (sorted_triangle.point_3.x - sorted_triangle.point_1.x) / (sorted_triangle.point_3.y - sorted_triangle.point_1.y);
            let second_length = (sorted_triangle.point_3.x - sorted_triangle.point_2.x) / (sorted_triangle.point_3.y - sorted_triangle.point_2.y);

            while index_y > sorted_triangle.point_1.y {
                unsafe {
                    target.DrawLine(
                        D2D_POINT_2F {
                            x: first_target_point_x,
                            y: index_y,
                        },
                        D2D_POINT_2F {
                            x: second_target_point_x,
                            y: index_y,
                        },
                        brush,
                        2.0,
                        &window.style,
                    );
                }

                /*let mut target_point = first_target_point_x;

                while target_point < second_target_point_x {
                    let rect = D2D_RECT_F {
                        left: target_point,
                        top: index_y,
                        right: target_point + 1.0,
                        bottom: index_y + 1.0,
                    };
                    unsafe {
                        target.FillRectangle(&rect, brush);
                    };

                    target_point += 1.0;
                }*/

                first_target_point_x -= first_length;
                second_target_point_x -= second_length;

                index_y -= 1.0;
            }
        }

        if self.is_visible {
            
            let sorted_triangle: Triangle = sort_triangle_points(&self);

            if sorted_triangle.point_2.y == sorted_triangle.point_3.y {
                fill_bottom_flat_triangle(window, sorted_triangle, &brush);
            } else if sorted_triangle.point_1.y == sorted_triangle.point_2.y {
                fill_top_flat_triangle(window, sorted_triangle, &brush);
            } else {
                let fourth: VectorPoint3D = VectorPoint3D {
                    x: (
                        sorted_triangle.point_1.x + 
                        ((sorted_triangle.point_2.y - sorted_triangle.point_1.y) / 
                        (sorted_triangle.point_3.y - sorted_triangle.point_1.y)) * 
                        (sorted_triangle.point_3.x - sorted_triangle.point_1.x)
                    ),
                    y: sorted_triangle.point_2.y,
                    z: sorted_triangle.point_2.z
                };
    
                let sort_for_bottom_flat_triangle = Triangle {
                    point_1: sorted_triangle.point_1.clone(),
                    point_2: sorted_triangle.point_2.clone(),
                    point_3: fourth.clone(),
                    is_visible: true
                };
    
                let sort_for_fill_top_flat_triangle = Triangle {
                    point_1: sorted_triangle.point_2.clone(),
                    point_2: fourth.clone(),
                    point_3: sorted_triangle.point_3.clone(),
                    is_visible: true
                };
    
                fill_bottom_flat_triangle(window, sort_for_bottom_flat_triangle, &brush);
                fill_top_flat_triangle(window, sort_for_fill_top_flat_triangle, &brush); 
            }
        }
    }

    pub fn draw_triangle(&self, window: &Window) {
        let target = window.target.as_ref().unwrap();
        let white_brush = window.white_brush.as_ref().unwrap();

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
                white_brush,
                2.0,
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
                white_brush,
                2.0,
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
                white_brush,
                2.0,
                &window.style,
            );
        }
    }
}