use components::transform::Transform;
use components::parent::Parent;
use specs::{System, ReadStorage, WriteStorage, Entities, Join, Entity};
use scene_tree::SceneTree;
use cgmath::Point3;

pub struct Transformation {
    scene_tree: SceneTree,
}

impl Transformation {
    pub fn new(entity: Entity) -> Self {
        Self {
            scene_tree: SceneTree::new(entity),
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
        self.scene_tree.reset();

        // Assign a scene node to every positioned entitiy.
        for (entity, _transform) in (&*entity, &tranform_storage).join() {
            self.scene_tree.add_entity_node(entity);
        }

        // Iterate over all entities with a parent component to create the scene tree.
        for (entity, _transform, parent) in (&*entity, &tranform_storage, &parent_storage).join() {
            self.scene_tree.set_entity_child(&parent.entity, &entity);
        }

        let root_entities = self.scene_tree.root_entities();

        for root_entity in root_entities {
            let descendant_entities = self.scene_tree.descendant_entities(&root_entity);

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
                    if transform.is_local_position_changed {
                        if let Some(parent_position) = parent_position_option {
                            transform.position = Point3 {
                                x: parent_position.x + transform.local_position.x,
                                y: parent_position.y + transform.local_position.y,
                                z: parent_position.z + transform.local_position.z,
                            };
                        } else {
                            transform.position = transform.local_position;
                        }

                        transform.is_local_position_changed = false;
                    } else if transform.is_position_changed {
                        if let Some(parent_position) = parent_position_option {
                            transform.local_position = Point3 {
                                x: parent_position.x + transform.position.x,
                                y: parent_position.y + transform.position.y,
                                z: parent_position.z + transform.position.z,
                            };
                        } else {
                            transform.local_position = transform.position;
                        }

                        transform.is_position_changed = false;
                    }
                }
            }
        }
    }
}
