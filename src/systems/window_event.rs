extern crate glutin;

use std::rc::Rc;
use specs::{System, Write};
use window::Window;
use input::Input;
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
        let dpi_factor = gl_window.get_hidpi_factor();

        input_ctx.new_tick();

        events_loop.poll_events(|event| {
            if let Event::WindowEvent{ event, .. } = event {
                match event {
                    GlutinWindowEvent::CloseRequested => *running = false,
                    GlutinWindowEvent::Resized(logical_size) => {
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    GlutinWindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput { state, virtual_keycode, .. } => {
                            if let Some(key) = virtual_keycode {
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
                        }
                    }
                    GlutinWindowEvent::CursorMoved { position, .. } => {
                        let logical_size = gl_window.get_inner_size().unwrap();
                        let window_center_x = logical_size.width / 2.0;
                        let window_center_y = logical_size.height / 2.0;

                        input_ctx.set_mouse_position(
                            (position.x, position.y),
                            (window_center_x, window_center_y),
                        );
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
                    } => input_ctx.set_mouse_wheel(y),
                    GlutinWindowEvent::MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(logical_position),
                        phase: TouchPhase::Moved,
                        ..
                    } => input_ctx.set_mouse_wheel(logical_position.y as f32),
                    _ => ()
                };
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
