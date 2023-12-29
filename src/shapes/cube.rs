use windows::{core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*, Win32::UI::WindowsAndMessaging::*};

use crate::engine_core::{Window};
use crate::shapes::primitives::{VectorPoint3D, Triangle};
use crate::math::{
    perspective_projection, 
    from_cartesian_to_screen_coordinates,
    Rotatin,
    RotationTypes
};
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct BuildedCube {
    pub points: Vec<VectorPoint3D>,
    pub indices: Vec<u16>,
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

        let indices = vec![
            0,1,2,
            3,2,1,
            6,5,4,
            5,6,7,
            4,1,0,
            1,4,5,
            2,3,6,
            7,6,3,
            0,2,4,
            6,4,2,
            5,3,1,
            3,5,7,
        ]; // розташовую точки куба таким чином, щоб потрібні трикутники були повернуті обличчям до камери, а інші відвернуті

        Self {
            points,
            indices,
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
    pub draw_as_triangles: Vec<Triangle>,
    pub use_triangles_for_build: bool,
    pub fill_in_triangles_with_color: bool,
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
            to_draw: Vec::new(),
            draw_as_triangles: Vec::new(),
            use_triangles_for_build: false,
            fill_in_triangles_with_color: false
        }
    }

    pub fn try_to_rotate_shape(&mut self, shape: &mut BuildedCube) {
        if self.rotation.is_need_rotate == true {
            self.rotation.iner_deley_counter += 1.0;
            
            if self.rotation.iner_deley_counter == self.rotation.deley_rotate_ms {

                self.rotation.iner_deley_counter = 0.0;

                let roatate_degree = 0.01;

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

    pub fn set_cube_as_triangles_from_points(&self, points: &Vec<VectorPoint3D>) -> Vec<Triangle> {
        let mut draw_as_triangles: Vec<Triangle> = Vec::new();

        let mut index = 0;

        while index < self.builded_cube.indices.len() {
            draw_as_triangles.push(
                Triangle::new().set_triangle(
                    points[self.builded_cube.indices[index] as usize].clone(),
                    points[self.builded_cube.indices[(index + 1) as usize] as usize].clone(),
                    points[self.builded_cube.indices[(index + 2) as usize] as usize].clone()
                )
            );

            index += 3;
        }

        draw_as_triangles
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

        let mut client_window_size = RECT::default();
        unsafe {
            GetClientRect(hwnd, &mut client_window_size);
        }

        self.try_to_rotate_shape(&mut builded_cube);

        let (vectors_with_perspective_projection, cube_2d_proection_on_screen): (Vec<VectorPoint3D>, Vec<VectorPoint3D>) = builded_cube.points.par_iter_mut().map(|vector| {
            // якщо треба обертати фігуру навколо себе, зміщення точок застосовуємо тільки після виклику функції обертання
            match self.rotation.rotation_type {
                RotationTypes::AroundSelf => {
                    vector.x += self.middle_dot_x;
                    vector.y += self.middle_dot_y;
                    vector.z += self.middle_dot_z;
                },
                _ => ()
            }
            
            // трансформуємо 3d вектор з урахуванням перспективи
            let vector_with_perspective_projection = perspective_projection(
                &vector,
                client_window_size.right as f32,
                client_window_size.bottom as f32
            );

            // трансформую вектор в екранні координати
            let result_vector = from_cartesian_to_screen_coordinates(
                &vector_with_perspective_projection, 
                client_window_size.right as f32,
                client_window_size.bottom as f32
            );

            (vector_with_perspective_projection, result_vector)
        }).collect();

        if self.use_triangles_for_build {
            /* 
                знаходжу паралелі для трикутників щоб визначити якою стороною повернутий трикутник
                (для використання формули яку я застосував, спочатку приводимо точки до перспективної проекції,
                для цього був сформований масив vectors_with_perspective_projection вище в коді, який тут використовую), 
                далі формую bool масив, де помічаю які з трикутників потрібно рендерити 
            */
            let triangles: Vec<Triangle> = self.set_cube_as_triangles_from_points(&vectors_with_perspective_projection);
            
            let is_visible_triangle: Vec<bool> = triangles.par_iter().map(|triangle| {
                if triangle.find_perpendicular_direction_to_triangle() < 0.0 {
                    return true;
                } else {
                    return false;
                }
            }).collect();

            /* 
                створюю фінальний масив трикутників за допомогою масиву точок "self.to_draw",
                в якому вже застосовано приведення до екранних координат,
                беру значення з масиву в якому вказано який з трикутників має бути відрендерений,
                і присвоюю їх фінальному масиву трикутників.
            */
            let mut draw_as_triangles = self.set_cube_as_triangles_from_points(&cube_2d_proection_on_screen);

            draw_as_triangles.par_iter_mut().enumerate().for_each(|(index, triangle)| {
                if triangle.is_visible != is_visible_triangle[index] {
                    triangle.is_visible = is_visible_triangle[index];
                }
            });

            self.draw_as_triangles = draw_as_triangles;
        } else {
            self.to_draw = cube_2d_proection_on_screen;
        }
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

    pub fn draw_cube_from_points(&self, window: &Window) {
        let target = window.target.as_ref().unwrap();
        let yellow_brush = window.yellow_brush.as_ref().unwrap();

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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
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
                yellow_brush,
                4.0,
                &window.style,
            );
        }
    }

    pub fn create_color(&self, target: &ID2D1DeviceContext, red: f32, green: f32, blue: f32, alfa: f32, opacity: f32) -> Result<ID2D1SolidColorBrush> {
        let color = D2D1_COLOR_F { r: red, g: green, b: blue, a: alfa };

        let properties = D2D1_BRUSH_PROPERTIES { opacity: opacity, transform: Matrix3x2::identity() };

        unsafe { target.CreateSolidColorBrush(&color, &properties) }
    }

    pub fn create_light_target(&self, position_x: f32, position_y: f32, position_z: f32) {

    } 

    pub fn draw_cube_from_triangles(&self, window: &Window) {
        if self.fill_in_triangles_with_color {

            let mut counter: f32 = 0.0;
            let mut color_num: usize = 0;

            /*
            let target = window.target.as_ref().unwrap();

            let mut color = self.create_color(&target, 0.3, 0.7, 0.7, 1.0, 1.0).ok();
            */

            let colors = vec![
                window.white_brush.as_ref().unwrap(),
                window.black_brush.as_ref().unwrap(),
                window.gray_brush.as_ref().unwrap(),
                window.yellow_brush.as_ref().unwrap(),
                window.brush_red.as_ref().unwrap(),
                window.brush_green.as_ref().unwrap(),
                window.brush_blue.as_ref().unwrap()
            ];

            self.draw_as_triangles.iter().for_each(|triangle| {
                //triangle.draw_triangle(window);
                triangle.fill_triangle_color(window, colors[color_num]);

                counter += 0.5;
                if counter.trunc() == counter {
                    color_num += 1;
                }
            });

        } else {
            self.draw_as_triangles.iter().for_each(|triangle| {
                triangle.draw_triangle(window);
            });
        }

        //color = None
    }
}