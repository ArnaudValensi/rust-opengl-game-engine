use vec_tree::VecTree;
use specs::Entity;

pub struct SceneGraph {
    tree: VecTree<Entity>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            tree: VecTree::new(),
        }
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self {
            tree: VecTree::new(),
        }
    }
}
