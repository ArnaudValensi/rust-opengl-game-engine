// extern crate glbs;
extern crate gl;
extern crate glutin;

use common::{Window, process_events};
use self::glutin::GlContext;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_1_2() {
    // glfw: initialize and configure
    // ------------------------------
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT);

    // render loop
    // -----------
    while window.running {
        // events
        // -----
        process_events(&mut window);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_window.swap_buffers().unwrap();
    }
}
