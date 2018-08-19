use components::transform::Transform;
use components::parent::Parent;
use specs::{System, ReadStorage, Entity, Entities, Join};
use vec_tree::{VecTree, NodeId};
use std::collections::HashMap;

pub struct Transformation {
    entity_node_map: HashMap<Entity, NodeId>,
    scene_tree: VecTree<Entity>,
}

impl Transformation {
    pub fn new() -> Self {
        Self {
            entity_node_map: HashMap::new(),
            scene_tree: VecTree::new(),
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

        // Clear the scene tree
        self.scene_tree.clear();
        self.entity_node_map.clear();

        // Assign a scene node to every positioned entitiy.
        for (entity, _transform) in (&*entity, &tranform_storage).join() {
            let scene_node = self.scene_tree.new_node(entity);

            self.entity_node_map.insert(entity, scene_node);
        }

        // Iterate over all entities with a parent component to create the scene tree.
        for (entity, _transform, parent) in (&*entity, &tranform_storage, &parent_storage).join() {
            let parent_entity = &parent.entity;
            let parent_scene_node = self.entity_node_map[parent_entity];
            let scene_node = self.entity_node_map[&entity];

            parent_scene_node.add_child(scene_node, &mut self.scene_tree);
        }

        // Get all entities with a transform.is_dirty.
        // Iterate over is and set the world tranform position/rotation/scale.
    }
}
