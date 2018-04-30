extern crate gl;
extern crate glutin;

use glutin::GlContext;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut events_loop = glutin::EventsLoop::new();
     let window = glutin::WindowBuilder::new()
         .with_title("Hello, world!")
         .with_dimensions(SCR_WIDTH, SCR_HEIGHT);
     let context = glutin::ContextBuilder::new()
         .with_vsync(true);
     let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

     unsafe {
         gl_window.make_current().unwrap();
     }

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    // render loop
    // -----------
    let mut running = true;
    while running {
        // events
        // -----
        process_events(&mut events_loop, &mut running, &gl_window);


        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        gl_window.swap_buffers().unwrap();
    }
}

fn process_events(events_loop: &mut glutin::EventsLoop, running: &mut bool, gl_window: &glutin::GlWindow) {
    events_loop.poll_events(|event| {
        match event {
            glutin::Event::WindowEvent{ event, .. } => match event {
                glutin::WindowEvent::Closed => *running = false,
                glutin::WindowEvent::Resized(width, height) => {
                    gl_window.resize(width, height);
                    // unsafe { gl::Viewport(0, 0, width, height) }
                },
                _ => ()
            },
            _ => ()
        }
    });
}
