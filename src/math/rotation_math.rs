use crate::shapes::{VectorPoint3D, BuildedCube};

#[derive(Clone, Debug)]
pub struct RotateDirections {
    pub rotate_by_x: bool,
    pub rotate_by_y: bool,
    pub rotate_by_z: bool,
}

impl RotateDirections {
    pub fn new() -> Self {
        Self {
            rotate_by_x: false,
            rotate_by_y: false,
            rotate_by_z: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Degree {
    pub degree_by_x: f32,
    pub degree_by_y: f32,
    pub degree_by_z: f32,
}

impl Degree {
    pub fn new() -> Self {
        Self {
            degree_by_x: 0.0,
            degree_by_y: 0.0,
            degree_by_z: 0.0
        }
    }
}
#[derive(Clone, Debug)]
pub struct Rotatin {
    pub is_need_rotate: bool,
    pub rotate_directions: RotateDirections,
    pub degree: Degree,
    pub deley_rotate_ms: f32,
    pub iner_deley_counter: f32,
}

impl Rotatin {
    pub fn new() -> Self {
        Self {
            is_need_rotate: false,
            degree: Degree::new(),
            rotate_directions: RotateDirections::new(),
            deley_rotate_ms: 10.0,
            iner_deley_counter: 0.0,
        }
    }

    pub fn rotation_by_x(&mut self, rotate_degree: f32) -> &mut Self {
        self.degree.degree_by_x += rotate_degree;

        if self.degree.degree_by_x > 6.283185307179586476925286766559 { // перевіряю чи зробив куб повний оберт, тобто чи більше число чим 2пі, що дорівнює 360 градусам, після чого обнуляю його значення, щоб число перед комою не росло до нескінченності, якщо цього не робити рано чи пізно станеться переповнення типу, программа запанікує і завершиться з помилкою
            self.degree.degree_by_x = 0.0;
        }

        self
    }
    
    pub fn rotation_by_y(&mut self, rotate_degree: f32) -> &mut Self {
        self.degree.degree_by_y += rotate_degree;

        if self.degree.degree_by_y > 6.283185307179586476925286766559 { // перевіряю чи зробив куб повний оберт, тобто чи більше число чим 2пі, що дорівнює 360 градусам, після чого обнуляю його значення, щоб число перед комою не росло до нескінченності, якщо цього не робити рано чи пізно станеться переповнення типу, программа запанікує і завершиться з помилкою
            self.degree.degree_by_y = 0.0;
        }

        self
    }
    
    pub fn rotation_by_z(&mut self, rotate_degree: f32) -> &mut Self {
        self.degree.degree_by_z += rotate_degree;

        if self.degree.degree_by_z > 6.283185307179586476925286766559 { // перевіряю чи зробив куб повний оберт, тобто чи більше число чим 2пі, що дорівнює 360 градусам, після чого обнуляю його значення, щоб число перед комою не росло до нескінченності, якщо цього не робити рано чи пізно станеться переповнення типу, программа запанікує і завершиться з помилкою
            self.degree.degree_by_z = 0.0;
        }

        self
    }
    
    pub fn rotate_shape(&mut self, shape: &mut BuildedCube) -> &Self {
        for point in &mut shape.points {
            self.rotate_point_by_rotate_matrix(point);
        }

        self
    }

    fn rotate_point_by_rotate_matrix(&self, point: &mut VectorPoint3D) -> &Self {
        let copy_point: VectorPoint3D = point.clone();

        point.x = copy_point.x * (self.degree.degree_by_z.cos() * self.degree.degree_by_y.cos()) +
            copy_point.y * (self.degree.degree_by_z.cos() * self.degree.degree_by_y.sin() * self.degree.degree_by_x.sin() - self.degree.degree_by_z.sin() * self.degree.degree_by_x.cos()) +
            copy_point.z * (self.degree.degree_by_z.cos() * self.degree.degree_by_y.sin() * self.degree.degree_by_x.cos() + self.degree.degree_by_z.sin() * self.degree.degree_by_x.sin());
    
        point.y = copy_point.x * (self.degree.degree_by_z.sin() * self.degree.degree_by_y.cos()) +
            copy_point.y * (self.degree.degree_by_z.sin() * self.degree.degree_by_y.sin() * self.degree.degree_by_x.sin() + self.degree.degree_by_z.cos() * self.degree.degree_by_x.cos()) +
            copy_point.z * (self.degree.degree_by_z.sin() * self.degree.degree_by_y.sin() * self.degree.degree_by_x.cos() - self.degree.degree_by_z.cos() * self.degree.degree_by_x.sin());
    
        point.z = copy_point.x * (-self.degree.degree_by_y.sin()) +
            copy_point.y * (self.degree.degree_by_y.cos() * self.degree.degree_by_x.sin()) +
            copy_point.z * (self.degree.degree_by_y.cos() * self.degree.degree_by_x.cos());
    
        self
    }
}