use windows::{core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*, Win32::UI::WindowsAndMessaging::*};

use crate::engine_core::{Window};
use crate::shapes::primitives::{VectorPoint3D, Triangle};
use crate::math::{
    perspective_projection, 
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
    pub draw_as_triangles: Vec<Triangle>
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
            to_draw: new_cube.to_draw,
            draw_as_triangles: new_cube.draw_as_triangles
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
            to_draw: Vec::new(),
            draw_as_triangles: Vec::new()
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
        // розташовую трикутники правильним чином, щоб потрібні трикутники були повернуті обличчям до камери
        let draw_as_triangles: Vec<Triangle> = vec![
            Triangle::new().set_triangle( // front side triangles
                points[0].clone(),
                points[1].clone(),
                points[2].clone()
            ),
            Triangle::new().set_triangle( // front side triangles
                points[3].clone(),
                points[2].clone(),
                points[1].clone(),
            ),
            Triangle::new().set_triangle( // back side triangles
                points[6].clone(),
                points[5].clone(),
                points[4].clone(),
            ),
            Triangle::new().set_triangle( // back side triangles
                points[5].clone(),
                points[6].clone(),
                points[7].clone(),
            ),
            Triangle::new().set_triangle( // top side triangles
                points[4].clone(),
                points[1].clone(),
                points[0].clone(),
            ),
            Triangle::new().set_triangle( // top side triangles
                points[1].clone(),
                points[4].clone(),
                points[5].clone(),
            ),
            Triangle::new().set_triangle( // down side triangles
                points[2].clone(),
                points[3].clone(),
                points[6].clone(),
            ),
            Triangle::new().set_triangle( // down side triangles
                points[7].clone(),
                points[6].clone(),
                points[3].clone(),
            ),
            Triangle::new().set_triangle( // left side triangles
                points[0].clone(),
                points[2].clone(),
                points[4].clone(),
            ),
            Triangle::new().set_triangle( // left side triangles
                points[6].clone(),
                points[4].clone(),
                points[2].clone(),
            ),
            Triangle::new().set_triangle( // right side triangles
                points[5].clone(),
                points[3].clone(),
                points[1].clone(),
            ),
            Triangle::new().set_triangle( // right side triangles
                points[3].clone(),
                points[5].clone(),
                points[7].clone(),
            ),
        ];

        draw_as_triangles

        //println!("{:?}", &self.draw_as_triangles);
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

        let mut vectors_with_perspective_projection: Vec<VectorPoint3D> = Vec::new();

        builded_cube.points.iter_mut().for_each(|vector| {
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

            vectors_with_perspective_projection.push(vector_with_perspective_projection);
            cube_2d_proection_on_screen.push(result_vector);
        });

        self.to_draw = cube_2d_proection_on_screen;

        /* 
            знаходжу паралелі для трикутників щоб визначити якою стороною повернутий трикутник
            (для використання формули яку я застосував, спочатку приводимо точки до перспективної проекції,
            для цього був сформований масив vectors_with_perspective_projection вище в коді, який тут використовую), 
            далі формую bool масив, де помічаю які з трикутників потрібно рендерити 
        */
        let triangles: Vec<Triangle> = self.set_cube_as_triangles_from_points(&vectors_with_perspective_projection);
        let mut is_visible_triangle: Vec<bool> = Vec::new();
        
        triangles.iter().for_each(|triangle| {
            if triangle.find_perpendicular_direction_to_triangle() < 0.0 {
                is_visible_triangle.push(true);
            } else {
                is_visible_triangle.push(false);
            }
        });

        /* 
            створюю фінальний масив трикутників за допомогою масиву точок "self.to_draw",
            в якому вже застосовано приведення до екранних координат,
            беру значення з масиву в якому вказано який з трикутників має бути відрендерений,
            і присвоюю їх фінальному масиву трикутників.
        */
        let mut draw_as_triangles_index = 0;
        let mut draw_as_triangles = self.set_cube_as_triangles_from_points(&self.to_draw);

        draw_as_triangles.iter_mut().for_each(|triangle| {
            if triangle.is_visible != is_visible_triangle[draw_as_triangles_index] {
                triangle.is_visible = is_visible_triangle[draw_as_triangles_index];
            }

            draw_as_triangles_index += 1;
        });

        self.draw_as_triangles = draw_as_triangles;
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
            if counter == 1.0 || counter == 2.0 || counter == 3.0 || counter == 4.0 || counter == 5.0 || counter == 6.0 {
                color_num += 1;
            }
        });

        //color = None
    }
}