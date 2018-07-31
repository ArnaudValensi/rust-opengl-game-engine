use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize,
}

impl NodeId {
    pub fn set_parent<T>(&self, parent_id: NodeId, tree: &Tree<T>) {
        // let current_node =
        // TODO
    }
}

#[derive(Debug)]
pub struct Node<T> {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    pub data: T,
}

impl<T> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parent: {:?}, ", self.parent)?;
        write!(f, "Previous sibling: {:?}, ", self.previous_sibling)?;
        write!(f, "Next sibling: {:?}, ", self.next_sibling)?;
        write!(f, "First child: {:?}, ", self.first_child)?;
        write!(f, "Last child: {:?}", self.last_child)
    }
}

impl<T> Node<T> {
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    pub fn first_child(&self) -> Option<NodeId> {
        self.first_child
    }

    pub fn last_child(&self) -> Option<NodeId> {
        self.last_child
    }

    pub fn previous_sibling(&self) -> Option<NodeId> {
        self.previous_sibling
    }

    pub fn next_sibling(&self) -> Option<NodeId> {
        self.next_sibling
    }
}

#[derive(Debug)]
pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { nodes: Vec::new() }
    }

    pub fn new_node(&mut self, data: T) -> NodeId {
        let index = self.get_free_index();

        self.nodes.push(Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            data: data,
        });

        NodeId { index }
    }

    fn get_free_index(&self) -> usize {
        self.nodes.len()
    }

    pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id.index)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id.index)
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn create_tree() {
         let mut tree = Tree::new();

         let root_id = tree.new_node(1);
         let child1_id = tree.new_node(2);
         let child2_id = tree.new_node(3);

         child1_id.set_parent(root_id, &tree);
         child2_id.set_parent(root_id, &tree);
        // assert!(!result2.is_err(), "it should not return an error");
    }
}
