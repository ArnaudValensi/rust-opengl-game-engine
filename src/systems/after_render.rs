extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use self::glutin::GlContext;
use specs::{System, Write};
use std::cell::RefCell;
use std::rc::Rc;
use resources::Time;
use window::Window;

pub struct AfterRender {
    window: Rc<RefCell<Window>>,
}

impl AfterRender {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        Self { window }
    }
}

impl<'a> System<'a> for AfterRender {
    type SystemData = Write<'a, Time>;

    fn run(&mut self, data: Self::SystemData) {
        let mut time = data;

        self.window.borrow().gl_window.swap_buffers().unwrap();
        time.frame_render_done();
    }
}
