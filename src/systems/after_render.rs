use specs::{System, Write};
use time::Time;

pub struct AfterRender;

impl AfterRender {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for AfterRender {
    type SystemData = Write<'a, Time>;

    fn run(&mut self, data: Self::SystemData) {
        let mut time = data;

        time.frame_render_done();
    }
}
