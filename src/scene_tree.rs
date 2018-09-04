use vec_tree::{
    VecTree,
    NodeId,
    Descendants,
    FollowingSiblings,
    Children,
    Ancestors,
    DescendantsWithDepth,
};
use specs::Entity;
use std::collections::HashMap;

pub struct SceneTree {
    pub tree: VecTree<Entity>,
    root_node: NodeId,
    scene_root_entity: Entity,
    entity_node_map: HashMap<Entity, NodeId>,
}

impl SceneTree {
    /// The scene_root_entity is needed to create the node which will hold the root nodes.
    pub fn new(scene_root_entity: Entity) -> Self {
        let mut tree = VecTree::new();
        let root_node = tree.new_node(scene_root_entity);

        Self {
            tree,
            root_node,
            scene_root_entity,
            entity_node_map: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.tree.clear();
        self.entity_node_map.clear();

        self.root_node = self.tree.new_node(self.scene_root_entity);
    }

    pub fn add_entity_node(&mut self, entity: Entity) {
        let scene_node = self.tree.new_node(entity);

        self.entity_node_map.insert(entity, scene_node);
        self.root_node.append_child(scene_node, &mut self.tree);
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
        node.append_child(child, &mut self.tree);
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

    /// Parent nodes appear before the descendants.
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn descendant_entities_with_depth(&self, entity: &Entity) -> DescendantEntitiesWithDepth {
        let node = self.get_entity_node(entity);

        DescendantEntitiesWithDepth {
            descendants_with_depth: node.descendants_with_depth(&self.tree),
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

    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn ancestor_entities(&self, entity: &Entity) -> AncestorEntities {
        let node = self.get_entity_node(entity);

        AncestorEntities {
            ancestors: node.ancestors(&self.tree),
            tree: &self.tree,
        }
    }

    /// This is the root nodes on the scene tree perspective. On the vec tree perspective, they are
    /// the children of the root node.
    pub fn root_entities(&self) -> ChildrenEntities {
        ChildrenEntities {
            children: self.root_node.children(&self.tree),
            tree: &self.tree,
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

pub struct DescendantEntitiesWithDepth<'a> {
    descendants_with_depth: DescendantsWithDepth<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for DescendantEntitiesWithDepth<'a> {
    type Item = (Entity, u32);

    fn next(&mut self) -> Option<(Entity, u32)> {
        let descendants_with_depth = &mut self.descendants_with_depth;

        match descendants_with_depth.next() {
            Some((node, depth)) => Some((self.tree[node].data, depth)),
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

pub struct AncestorEntities<'a> {
    ancestors: Ancestors<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for AncestorEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let ancestors = &mut self.ancestors;

        match ancestors.next() {
            Some(node) => Some(self.tree[node].data),
            None => None,
        }
    }
}

pub struct ChildrenEntities<'a> {
    children: Children<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for ChildrenEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let children = &mut self.children;

        match children.next() {
            Some(node) => Some(self.tree[node].data),
            None => None,
        }
    }
}
