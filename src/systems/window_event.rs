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
    KeyboardInput,
    MouseButton,
    MouseScrollDelta,
    TouchPhase,
};
use self::glutin::ElementState::{Pressed, Released};

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

        input_ctx.new_tick();

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
                                    Pressed => {
                                        input_ctx.set_key_down(normalized_key);
                                    }
                                    Released => {
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

                        input_ctx.set_mouse_position(position, (window_center_x, window_center_y));
                    }
                    GlutinWindowEvent::MouseInput { state, button, .. } => {
                        match button {
                            MouseButton::Left => input_ctx.set_mouse_left(state == Pressed),
                            MouseButton::Right => input_ctx.set_mouse_right(state == Pressed),
                            MouseButton::Middle => input_ctx.set_mouse_middle(state == Pressed),
                            _ => {}
                        }
                    }
                    GlutinWindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } |
                    GlutinWindowEvent::MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => input_ctx.set_mouse_wheel(y),
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
