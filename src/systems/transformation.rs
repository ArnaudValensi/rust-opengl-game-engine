use components::transform::Transform;
use components::parent::Parent;
use specs::{System, ReadStorage, WriteStorage, Entities, Join, Entity};
use scene_tree::SceneTree;
use cgmath::Matrix4;
use math::point_to_vector;

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

        let root_entities = {
            match self.scene_tree.root_entities() {
                Some(root_entities) => root_entities,
                _ => {
                    return;
                }
            }
        };

        let mut current_higher_dirty_depth: u32 = 0;

        for root_entity in root_entities {
            let descendant_entities = self.scene_tree.descendant_entities_with_depth(&root_entity);

            for (entity, depth) in descendant_entities {
                let parent_world_matrix_option = if let Some(parent_entity) = self.scene_tree.get_parent_entity(&entity) {
                    if let Some(parent_transform) = tranform_storage.get(*parent_entity) {
                        Some(parent_transform.world_matrix)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(transform) = tranform_storage.get_mut(entity) {
                    let is_dirty = transform.is_dirty || depth > current_higher_dirty_depth;

                    if is_dirty {
                        let local_rotation = Matrix4::from(transform.local_rotation);
                        let translation = Matrix4::from_translation(point_to_vector(transform.local_position));
                        let local_matrix: Matrix4<f32> = translation * local_rotation;

                        transform.local_matrix = local_matrix;

                        if let Some(parent_world_matrix) = parent_world_matrix_option {
                            transform.world_matrix = parent_world_matrix * local_matrix;
                        } else {
                            transform.world_matrix = local_matrix;
                        }

                        transform.is_dirty = false;
                        current_higher_dirty_depth = depth;
                    }
                }
            }
        }
    }
}
