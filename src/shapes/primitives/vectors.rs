#[derive(Debug, Clone)]
pub struct VectorPoint2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct VectorPoint3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl VectorPoint2D {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl VectorPoint3D {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}