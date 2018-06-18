use specs::System;

struct Render;

impl<'a> System<'a> for Render {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {}
}
