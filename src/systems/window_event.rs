extern crate glutin;

use std::rc::Rc;
use specs::{System, Write};
use window::Window;
use input::input::Input;
use input::keycode_normalizer::normalize_key;
use std::cell::RefCell;
use self::glutin::{
    Event,
    WindowEvent as GlutinWindowEvent,
    GlContext,
    ElementState,
    KeyboardInput,
};

// NOTE: WindowEvent must be run on the main thread.
pub struct WindowEvent {
    window: Rc<RefCell<Window>>,
}

impl WindowEvent {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        Self {
            window,
        }
    }

    pub fn process_events(&mut self, input_ctx: &mut Input) {
        let mut borrowed_window = self.window.borrow_mut();
        let tmp = &mut *borrowed_window;
        let events_loop = &mut tmp.events_loop;
        let running = &mut tmp.running;
        let gl_window = &tmp.gl_window;

        // input_ctx.new_tick();

        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent{ event, .. } => match event {
                    GlutinWindowEvent::Closed => *running = false,
                    GlutinWindowEvent::Resized(width, height) => {
                        gl_window.resize(width, height);
                    },
                    GlutinWindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput { state, virtual_keycode, .. } => match virtual_keycode {
                            Some(key) => {
                                let normalized_key = normalize_key(key);

                                match state {
                                    ElementState::Pressed => {
                                        input_ctx.set_key_down(normalized_key);
                                        println!("normalized_key: {:?}", normalized_key);
                                    }
                                    ElementState::Released => {
                                        input_ctx.set_key_up(normalized_key);
                                    }
                                }
                            }
                            _ => ()
                        }
                    }
                    GlutinWindowEvent::CursorMoved { position, .. } => {
                        let window_size = gl_window.get_inner_size().unwrap();
                        let window_center_x = window_size.0 as f64 / 2.0;
                        let window_center_y = window_size.1 as f64 / 2.0;

                        input_ctx.mouse_axis = (
                            position.0 - window_center_x,
                            position.1 - window_center_y
                        );

                        Window::center_mouse_cursor(gl_window);
                    }
                    _ => ()
                },
                _ => ()
            }
        });
    }
}

impl<'a> System<'a> for WindowEvent {
    type SystemData = Write<'a, Input>;

    fn run(&mut self, mut ctx_input: Self::SystemData) {
        self.process_events(&mut ctx_input);
    }
}
