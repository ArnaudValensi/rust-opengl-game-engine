extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use specs::{System, Read};
use window::Window;
use time::Time;
use self::glutin::GlContext;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SwapFrameBuffer {
    window: Rc<RefCell<Window>>,
}

impl SwapFrameBuffer {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        Self {
            window,
        }
    }
}

impl<'a> System<'a> for SwapFrameBuffer {
    type SystemData = (
        Read<'a, Time>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        self.window.borrow().gl_window.swap_buffers().unwrap();
    }
}
