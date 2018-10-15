extern crate vec_tree;
use self::vec_tree::{
    AncestorsIter, ChildrenIter, DescendantsIter, DescendantsWithDepthIter, FollowingSiblingsIter,
    Index, VecTree,
};
use specs::Entity;
use std::collections::HashMap;

pub struct SceneTree {
    pub tree: VecTree<Entity>,
    root_node: Index,
    scene_root_entity: Entity,
    entity_node_map: HashMap<Entity, Index>,
}

impl SceneTree {
    /// The scene_root_entity is needed to create the node which will hold the root nodes.
    pub fn new(scene_root_entity: Entity) -> Self {
        let mut tree = VecTree::new();
        let root_node = tree.insert_root(scene_root_entity);

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

        self.root_node = self.tree.insert_root(self.scene_root_entity);
    }

    pub fn add_entity_node(&mut self, entity: Entity) {
        let scene_node = self.tree.insert(entity, self.root_node);

        self.entity_node_map.insert(entity, scene_node);
    }

    pub fn get_entity_node(&self, entity: &Entity) -> Index {
        self.entity_node_map[entity]
    }

    pub fn set_entity_child(&mut self, entity: &Entity, child: &Entity) {
        let node = self.get_entity_node(entity);
        let child_node = self.get_entity_node(child);

        self.set_node_child(node, child_node);
    }

    fn set_node_child(&mut self, node: Index, child: Index) {
        self.tree.append_child(node, child);
    }

    pub fn get_parent_entity(&self, entity: &Entity) -> Option<&Entity> {
        let node = self.get_entity_node(entity);

        match self.tree.parent(node) {
            Some(node_id) => self.tree.get(node_id),
            _ => None,
        }
    }

    // pub fn get_entity_root_parent(&self, entity: &Entity) -> &Entity {
    //     let mut node_id: Index = self.get_entity_node(entity);

    //     loop {
    //         if let Some(parent_id) = node.parent() {
    //             node_id = parent_id;
    //         } else {
    //             return &self.tree[node_id].data;
    //         }
    //     }
    // }

    /// Parent nodes appear before the descendants.
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn descendant_entities(&self, entity: &Entity) -> DescendantEntities {
        let node_id = self.get_entity_node(entity);

        DescendantEntities {
            descendants: self.tree.descendants(node_id),
            tree: &self.tree,
        }
    }

    /// Parent nodes appear before the descendants.
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn descendant_entities_with_depth(&self, entity: &Entity) -> DescendantEntitiesWithDepth {
        let node_id = self.get_entity_node(entity);

        DescendantEntitiesWithDepth {
            descendants_with_depth: self.tree.descendants_with_depth(node_id),
            tree: &self.tree,
        }
    }

    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn following_entities(&self, entity: &Entity) -> FollowingEntities {
        let node_id = self.get_entity_node(entity);

        FollowingEntities {
            following_siblings: self.tree.following_siblings(node_id),
            tree: &self.tree,
        }
    }

    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn ancestor_entities(&self, entity: &Entity) -> AncestorEntities {
        let node_id = self.get_entity_node(entity);

        AncestorEntities {
            ancestors: self.tree.ancestors(node_id),
            tree: &self.tree,
        }
    }

    /// This is the root nodes on the scene tree perspective. On the vec tree perspective, they are
    /// the children of the root node.
    pub fn root_entities(&self) -> Option<ChildrenEntities> {
        let root_node_id_opt = self.tree.get_root_index();

        match root_node_id_opt {
            Some(root_node_id) => Some(ChildrenEntities {
                children: self.tree.children(root_node_id),
                tree: &self.tree,
            }),
            _ => None,
        }
    }
}

// TODO: DescendantEntitiesIter.
pub struct DescendantEntities<'a> {
    descendants: DescendantsIter<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for DescendantEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let descendants = &mut self.descendants;

        match descendants.next() {
            Some(node_id) => Some(self.tree[node_id]),
            None => None,
        }
    }
}

pub struct DescendantEntitiesWithDepth<'a> {
    descendants_with_depth: DescendantsWithDepthIter<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for DescendantEntitiesWithDepth<'a> {
    type Item = (Entity, u32);

    fn next(&mut self) -> Option<(Entity, u32)> {
        let descendants_with_depth = &mut self.descendants_with_depth;

        match descendants_with_depth.next() {
            Some((node_id, depth)) => Some((self.tree[node_id], depth)),
            None => None,
        }
    }
}

pub struct FollowingEntities<'a> {
    following_siblings: FollowingSiblingsIter<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for FollowingEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let following_siblings = &mut self.following_siblings;

        match following_siblings.next() {
            Some(node_id) => Some(self.tree[node_id]),
            None => None,
        }
    }
}

pub struct AncestorEntities<'a> {
    ancestors: AncestorsIter<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for AncestorEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let ancestors = &mut self.ancestors;

        match ancestors.next() {
            Some(node_id) => Some(self.tree[node_id]),
            None => None,
        }
    }
}

pub struct ChildrenEntities<'a> {
    children: ChildrenIter<'a, Entity>,
    tree: &'a VecTree<Entity>,
}

impl<'a> Iterator for ChildrenEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        let children = &mut self.children;

        match children.next() {
            Some(node_id) => Some(self.tree[node_id]),
            None => None,
        }
    }
}
