use windows;
use crate::shapes::{Cube};
use rayon::prelude::*;

pub struct LastPressKey {
    pub keystrokes: i32
}

impl LastPressKey {
    pub fn new() -> Self {
        Self {
            keystrokes: -1000
        }
    }

    pub fn check_keystrokes(&mut self, cubes: &mut Vec<Cube>) {
        unsafe {
            if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(87)
            == (1 | -32767 | -32768)
            {
                self.keystrokes = 87;
                //Up

                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_y += 20.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(83)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 83;
                //Down

                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_y -= 20.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(65)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 65;
                //Left

                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_x -= 20.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(68)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 68;
                //Right

                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_x += 20.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(38) == (1 | -32767 | -32768) {
                self.keystrokes = 38;
                // camera move forward
                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_z -= 10.0;

                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(40) == (1 | -32767 | -32768) {
                self.keystrokes = 40;
                // camera move back
                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_z += 10.0;

                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(37) == (1 | -32767 | -32768) {
                self.keystrokes = 37;
                // camera move left
                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_x += 10.0;

                    /*let roatate_degree = 0.05;
                    cube.rotation.rotation_by_y(roatate_degree);*/

                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(39) == (1 | -32767 | -32768) {
                self.keystrokes = 39;
                // camera move right
                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_x -= 10.0;

                    /*let roatate_degree = -0.05;
                    cube.rotation.rotation_by_y(roatate_degree);*/

                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(90)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 90;
                //Z axis

                cubes.par_iter_mut().for_each(|cube| {
                    cube.rotation.rotate_directions.rotate_by_z = !cube.rotation.rotate_directions.rotate_by_z;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(88)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 88;
                //X axis

                cubes.par_iter_mut().for_each(|cube| {
                    cube.rotation.rotate_directions.rotate_by_x = !cube.rotation.rotate_directions.rotate_by_x;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(89)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 89;
                //Y axis

                cubes.par_iter_mut().for_each(|cube| {
                    cube.rotation.rotate_directions.rotate_by_y = !cube.rotation.rotate_directions.rotate_by_y;
                    cube.builded_cube.is_builded = false;
                });
            }
            
            else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(13)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 13;
                //Enter

                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_z += 20.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(32) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(27)
                == (1 | -32767 | -32768)
            {
                self.keystrokes = 32;
                //Space

                cubes.par_iter_mut().for_each(|cube| {
                    cube.middle_dot_z -= 20.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(107) == (1 | -32767 | -32768) {
                self.keystrokes = 107;

                cubes.par_iter_mut().for_each(|cube| {
                    cube.size += 1.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(109) == (1 | -32767 | -32768) {
                self.keystrokes = 109;

                cubes.par_iter_mut().for_each(|cube| {
                    cube.size -= 1.0;
                    cube.builded_cube.is_builded = false;
                });
            } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(80) == (1 | -32767 | -32768) {
                self.keystrokes = 80;

                cubes.par_iter_mut().for_each(|cube| {
                    cube.rotation.is_need_rotate = !cube.rotation.is_need_rotate;
                });
            }
        }
    }
}