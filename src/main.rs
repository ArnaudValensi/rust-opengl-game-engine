extern crate glbs;
extern crate gl;
extern crate glutin;

use glbs::common;
use glutin::GlContext;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct Window {
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow,
}

impl Window {
    pub fn new() -> Window {
        let events_loop = glutin::EventsLoop::new();
         let window = glutin::WindowBuilder::new()
             .with_title("Hello, world!")
             .with_dimensions(SCR_WIDTH, SCR_HEIGHT);
         let context = glutin::ContextBuilder::new()
             .with_vsync(true);
         let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

         Window {
             events_loop,
             gl_window,
         }
    }
}

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut window = Window::new();

     unsafe {
         window.gl_window.make_current().unwrap();
     }

    gl::load_with(|symbol| window.gl_window.get_proc_address(symbol) as *const _);

    // render loop
    // -----------
    let mut running = true;
    while running {
        // events
        // -----
        common::process_events(&mut window.events_loop, &mut running, &window.gl_window);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_window.swap_buffers().unwrap();
    }
}
