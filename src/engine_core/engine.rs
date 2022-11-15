use windows::{core::*, Win32::System::Com::*};
use crate::engine_core::win::{Window};
use crate::shapes::{Cube, Shape};

#[derive(Clone)]
pub enum Statuses {
    Waiting,
    Runed,
    Paused
}

#[derive(Clone)]
pub struct Engine {
    status: Statuses,
    fps: i32,
    cubes: Vec<Cube>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            status: Statuses::Waiting,
            fps: 0,
            cubes: Vec::new()
        }
    }

    pub fn build_cube(
        &mut self, 
        cube: Cube
    ) -> &mut Self {
        match self.status {
            Statuses::Waiting | Statuses::Paused => {
                self.cubes.push(cube);
                self
            },
            Statuses::Runed => panic!("
                'run()' need called in end of functions from Engine.
                for example: Engine::new().build_cube(your_cube)).run();
            "),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        unsafe {
            CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)?;
        }

        let mut window = Window::new()?;

        self.status = Statuses::Runed;

        window.run(self.cubes.clone())
    }
}