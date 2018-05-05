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
    CursorState,
};

pub struct Window {
    pub gl_window: GlWindow,
    pub input: Input,
    pub running: bool,
    pub events_loop: EventsLoop,
    cursor_locked: bool,
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

        gl_window.set_cursor_state(CursorState::Grab);

        unsafe {
            gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        Window {
            events_loop,
            input,
            gl_window,
            running: true,
            cursor_locked: true,
        }
    }

    pub fn set_cursor_locked(&mut self, lock: bool) {
        self.cursor_locked = lock;
    }
}

pub fn process_events(window: &mut Window) {
    let running = &mut window.running;
    let gl_window = &window.gl_window;
    let window_input = &mut window.input;

    window_input.mouse_axis = (0.0, 0.0);

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
                    let window_size = gl_window.get_inner_size().unwrap();
                    let window_center_x = window_size.0 as f64 / 2.0;
                    let window_center_y = window_size.1 as f64 / 2.0;

                    window_input.mouse_axis = (
                        position.0 - window_center_x,
                        position.1 - window_center_y
                    );
                    // println!("window_size: {:?}", window_size);

                    // println!("window_center_x: {:?}", window_center_x);
                    // println!("window_center_y: {:?}", window_center_y);
                    // println!("position: {:?}", position);
                    // println!("window_input.mouse_axis: {:?}", window_input.mouse_axis);
                    center_mouse_cursor(gl_window);
                }
                _ => ()
            },
            _ => ()
        }
    });
}

fn center_mouse_cursor(gl_window: &GlWindow) {
    let hidpi_factor = gl_window.hidpi_factor() as i32;
    let window_size =  gl_window.get_inner_size().unwrap();
    let posx = window_size.0 as i32 / hidpi_factor / 2;
    let posy = window_size.1 as i32 / hidpi_factor / 2;

    gl_window.set_cursor_position(posx, posy);
}
