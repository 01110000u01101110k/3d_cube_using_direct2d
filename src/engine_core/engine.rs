use windows::{core::*, Win32::System::Com::*};
use crate::engine_core::win::{Window};
use crate::shapes::{Cube, Shape};
use rayon::prelude::*;

pub fn process_build_shapes(window: &mut Window) {
    window.cubes.iter_mut().for_each(|cube| {
        cube.build_shape(window.handle);
    });
}

pub fn process_draw(window: &mut Window) {
    window.cubes.iter().for_each(|cube| {
        if cube.use_triangles_for_build {
            cube.draw_cube_from_triangles(window);
        } else {
            cube.draw_cube_from_points(window);
        }
        
    });
}

pub enum Statuses {
    Waiting,
    Runed,
    Paused
}

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

        window.run(process_build_shapes, process_draw, self.cubes.clone())
    }
}