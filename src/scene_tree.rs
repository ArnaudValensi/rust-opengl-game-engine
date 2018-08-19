use vec_tree::{VecTree, NodeId};
use specs::Entity;
use std::collections::HashMap;

pub struct SceneTree {
    pub tree: VecTree<Entity>,
    entity_node_map: HashMap<Entity, NodeId>,
}

impl SceneTree {
    pub fn new() -> Self {
        Self {
            tree: VecTree::new(),
            entity_node_map: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.tree.clear();
        self.entity_node_map.clear();
    }

    pub fn add_entity_node(&mut self, entity: Entity) {
        let scene_node = self.tree.new_node(entity);

        self.entity_node_map.insert(entity, scene_node);
    }

    pub fn get_entity_node(&self, entity: &Entity) -> NodeId {
        self.entity_node_map[entity]
    }

    pub fn set_entity_child(&mut self, entity: &Entity, child: &Entity) {
        let node = self.get_entity_node(entity);
        let child_node = self.get_entity_node(child);

        self.set_node_child(node, child_node);
    }

    fn set_node_child(&mut self, node: NodeId, child: NodeId) {
        node.add_child(child, &mut self.tree);
    }
}

impl Default for SceneTree {
    fn default() -> Self {
        Self {
            tree: VecTree::new(),
            entity_node_map: HashMap::new(),
        }
    }
}
