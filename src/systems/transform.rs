use components::transform::Transform;
use scene_graph::SceneGraph;
use specs::{System, Read, ReadStorage};

pub struct Transformation {

}

impl Transformation {
    pub fn new() -> Self {
        Self {

        }
    }
}

/// This system is meant to compute the local transform based on the parent transform.
impl<'a> System<'a> for Transformation {
    type SystemData = (
        ReadStorage<'a, Transform>,
        Read<'a, SceneGraph>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_tranform_storage, _scene_graph) = data;
    }
}
