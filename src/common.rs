extern crate glutin;

use self::glutin::{Event, WindowEvent, EventsLoop, GlWindow, GlContext};

pub fn process_events(events_loop: &mut EventsLoop, running: &mut bool, gl_window: &GlWindow) {
    events_loop.poll_events(|event| {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::Closed => *running = false,
                WindowEvent::Resized(width, height) => {
                    gl_window.resize(width, height);
                    // unsafe { gl::Viewport(0, 0, width, height) }
                },
                _ => ()
            },
            _ => ()
        }
    });
}
