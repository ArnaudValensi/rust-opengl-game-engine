use components::transform::Transform;
use components::parent::Parent;
use specs::{System, ReadStorage, Entities, Join};
use scene_tree::SceneTree;

pub struct Transformation {
    scene_tree: SceneTree,
}

impl Transformation {
    pub fn new() -> Self {
        Self {
            scene_tree: SceneTree::new(),
        }
    }
}

/// This system is meant to compute the local transform based on the parent transform.
impl<'a> System<'a> for Transformation {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Parent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entity, tranform_storage, parent_storage) = data;

        // Clear the scene tree.
        self.scene_tree.clear();

        // Assign a scene node to every positioned entitiy.
        for (entity, _transform) in (&*entity, &tranform_storage).join() {
            self.scene_tree.add_entity_node(entity);
        }

        // Iterate over all entities with a parent component to create the scene tree.
        for (entity, _transform, parent) in (&*entity, &tranform_storage, &parent_storage).join() {
            self.scene_tree.set_entity_child(&parent.entity, &entity);
        }

        // Get all entities with a transform.is_dirty.
        // Iterate over is and set the world tranform position/rotation/scale.
    }
}
