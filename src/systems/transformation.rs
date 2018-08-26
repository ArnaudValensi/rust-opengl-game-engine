use components::transform::Transform;
use components::parent::Parent;
use specs::{System, ReadStorage, WriteStorage, Entities, Join};
use scene_tree::SceneTree;
use cgmath::Point3;

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
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Parent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entity, mut tranform_storage, parent_storage) = data;

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

        if let Some(first_root_entity) = self.scene_tree.get_first_root_entity() {
            // TODO: iterate over all siblings
            // let sibling_entities = self.scene_tree.following_entities(first_root_entity);

            // When world transform is set, set is_dirty to false
            let descendant_entities = self.scene_tree.descendant_entities(first_root_entity);

            for entity in descendant_entities {
                let parent_position_option = if let Some(parent_entity) = self.scene_tree.get_parent_entity(&entity) {
                    if let Some(parent_transform) = tranform_storage.get(*parent_entity) {
                        Some(parent_transform.position)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(transform) = tranform_storage.get_mut(entity) {
                    if transform.is_dirty {
                        if let Some(parent_position) = parent_position_option {
                            transform.position = Point3 {
                                x: parent_position.x + transform.local_position.x,
                                y: parent_position.y + transform.local_position.y,
                                z: parent_position.z + transform.local_position.z,
                            };
                        } else {
                            println!("transformation: {:#?}", transform.position);

                            transform.position = transform.local_position;
                        }

                        transform.is_dirty = false;
                        println!("is_dirty fasle");
                    }
                }
            }
        }
    }
}
