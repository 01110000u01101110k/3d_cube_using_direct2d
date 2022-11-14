use windows::{core::*};

use crate::engine_core::{Window};

pub enum Shapes {
    Cube,
    Circle,
    Any
}

pub trait Shape {
    fn new() -> Self;

    fn build_shape(&self, window: Window);
}