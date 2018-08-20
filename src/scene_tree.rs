use vec_tree::{VecTree, NodeId};
use specs::Entity;
use std::collections::HashMap;
use vec_tree::{Descendants, FollowingSiblings};

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

    pub fn get_first_root_entity(&self) -> Option<&Entity> {
        match self.tree.get_first_root() {
            Some(node) => Some(&self.tree[node].data),
            None => None,
        }
    }

    pub fn get_parent_entity(&self, entity: &Entity) -> Option<&Entity> {
        let node = self.get_entity_node(entity);

        match self.tree[node].parent() {
            Some(node) => Some(&self.tree[node].data),
            None => None,
        }
    }

    pub fn get_entity_root_parent(&self, entity: &Entity) -> &Entity {
        let mut node_id: NodeId = self.get_entity_node(entity);

        loop {
            let node = &self.tree[node_id];

            if let Some(parent_id) = node.parent() {
                node_id = parent_id;
            } else {
                return &self.tree[node_id].data;
            }
        }
    }

    /// Parent nodes appear before the descendants.
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn descendant_entities(&self, entity: &Entity) -> DescendantEntities {
        let node = self.get_entity_node(entity);

        DescendantEntities {
            descendants: node.descendants(&self.tree),
            tree: &self.tree,
        }
    }

    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn following_entities(&self, entity: &Entity) -> FollowingEntities {
        let node = self.get_entity_node(entity);

        FollowingEntities {
            following_siblings: node.following_siblings(&self.tree),
            tree: &self.tree,
        }
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

pub struct DescendantEntities<'a> {
    descendants: Descendants<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for DescendantEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let descendants = &mut self.descendants;

        match descendants.next() {
            Some(node) => Some(self.tree[node].data),
            None => None,
        }
    }
}

pub struct FollowingEntities<'a> {
    following_siblings: FollowingSiblings<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for FollowingEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let following_siblings = &mut self.following_siblings;

        match following_siblings.next() {
            Some(node) => Some(self.tree[node].data),
            None => None,
        }
    }
}
