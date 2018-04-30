extern crate glutin;
extern crate gl;

use self::glutin::{
    WindowBuilder,
    ContextBuilder,
    Event,
    WindowEvent,
    EventsLoop,
    GlWindow,
    GlContext
};

pub struct Window {
    pub gl_window: glutin::GlWindow,
    pub running: bool,
    events_loop: glutin::EventsLoop,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let events_loop = EventsLoop::new();
         let window = WindowBuilder::new()
             .with_title("Hello, world!")
             .with_dimensions(width, height);
         let context = ContextBuilder::new()
             .with_vsync(true);
         let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

         unsafe {
             gl_window.make_current().unwrap();
         }

         gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

         Window {
             events_loop,
             gl_window,
             running: true,
         }
    }
}

pub fn process_events(window: &mut Window) {
    let running = &mut window.running;
    let gl_window = &window.gl_window;

    window.events_loop.poll_events(|event| {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::Closed => *running = false,
                WindowEvent::Resized(width, height) => {
                    gl_window.resize(width, height);
                },
                _ => ()
            },
            _ => ()
        }
    });
}
