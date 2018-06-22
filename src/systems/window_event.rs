use std::rc::Rc;
use specs::{System, Read};
use window::Window;
use input::input::Input;

pub struct WindowEvent {
    window: Rc<Window>,
}

impl WindowEvent {
    pub fn new(window: Rc<Window>) -> Self {
        Self {
            window,
        }
    }
}

impl<'a> System<'a> for WindowEvent {
    type SystemData = Read<'a, Input>;

    fn run(&mut self, mut ctx_input: Self::SystemData) {
    }
}
