extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use specs::{System, Write};
use window::Window;
use input::input::Input;
use input::keyboard::KeyCode;
use std::rc::Rc;
use std::cell::RefCell;
use self::glutin::CursorState;

pub struct MouseControl {
    window: Rc<RefCell<Window>>,
}

impl MouseControl {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        Self {
            window,
        }
    }
}

impl<'a> System<'a> for MouseControl {
    type SystemData = (
        Write<'a, Input>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input,) = data;
        let gl_window = &self.window.borrow().gl_window;

        if input.get_key_down(KeyCode::Escape) {
            input.toggle_cursor_lock();
            println!("input.is_cursor_locked(): {:?}", input.is_cursor_locked());

            if input.is_cursor_locked() {
                gl_window
                    .set_cursor_state(CursorState::Grab)
                    .ok()
                    .expect("could not grab mouse cursor");
            } else {
                gl_window
                    .set_cursor_state(CursorState::Normal)
                    .ok()
                    .expect("could not free mouse cursor");
            }
        }

        if input.is_cursor_locked() {
            Window::center_mouse_cursor(gl_window);
        }

    }
}
