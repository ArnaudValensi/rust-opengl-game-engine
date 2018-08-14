use tree::Tree;
use specs::Entity;

pub struct SceneGraph {
    tree: Tree<Entity>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            tree: Tree::new(),
        }
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self {
            tree: Tree::new(),
        }
    }
}
