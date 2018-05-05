extern crate glutin;
extern crate gl;

use input::input::Input;
use input::keycode_normalizer::normalize_key;
use self::glutin::{
    WindowBuilder,
    ContextBuilder,
    Event,
    WindowEvent,
    EventsLoop,
    GlWindow,
    GlContext,
    ElementState,
    KeyboardInput,
};

pub struct Window {
    pub gl_window: glutin::GlWindow,
    pub input: Input,
    pub running: bool,
    pub events_loop: glutin::EventsLoop,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let events_loop = EventsLoop::new();
        let input = Input::new();
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
            input,
            gl_window,
            running: true,
        }
    }
}

pub fn process_events(window: &mut Window) {
    let running = &mut window.running;
    let gl_window = &window.gl_window;
    let window_input = &mut window.input;

    window.events_loop.poll_events(|event| {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::Closed => *running = false,
                WindowEvent::Resized(width, height) => {
                    gl_window.resize(width, height);
                },
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput { state, virtual_keycode, .. } => match virtual_keycode {
                        Some(key) => {
                            let normalized_key = normalize_key(key);

                            match state {
                                ElementState::Pressed => {
                                    window_input.set_key_down(normalized_key);
                                }
                                ElementState::Released => {
                                    window_input.set_key_up(normalized_key);
                                }
                            }
                        }
                        _ => ()
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    window_input.mouse_position = position;
                }
                _ => ()
            },
            _ => ()
        }
    });
}
