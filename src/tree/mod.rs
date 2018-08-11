use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize,
}

// |---|
// | A |
// |---|
//
// |---|  |---|
// | B |  | C |
// |---|  |---|
impl NodeId {
    // unlink

    // self.parent = parent

    // if self.parent.first_child != None {
    //   self.parent.first_child.next_sibling = self;
    //   self.previous_sibling = self.parent.first_child;
    // }

    // if self.parent.first_child != None && self.parent.first_child == self.parent.last_child { self.parent.first_child = self; self.parent.last_child = self; }
    // else self.parent.last_child = self;

    // self.previous_sibling = parent.last_child
    pub fn set_parent<T>(&self, parent_id: NodeId, tree: &mut Tree<T>) {
        // let self_node = {
        //     tree.get_mut(*self).unwrap()
        // };
        //
        // // TODO: remove links
        //
        // let parent_node = tree.get(parent_id).unwrap();
        //
        // self_node.parent = Some(parent_id);
        //
        // if let Some(parent_first_child_id) = parent_node.first_child {
        //     let mut parent_first_child = tree.get_mut(parent_first_child_id).unwrap();
        //
        //     parent_first_child.next_sibling = Some(*self);
        //     self_node.previous_sibling = Some(parent_first_child_id);
        // }
    }

    pub fn detach<T>(self, tree: &mut Tree<T>) {
        let (parent, previous_sibling, next_sibling) = {
            let node = &mut tree[self];
            (
                node.parent.take(),
                node.previous_sibling.take(),
                node.next_sibling.take(),
            )
        };

        if let Some(next_sibling) = next_sibling {
            tree[next_sibling].previous_sibling = previous_sibling;
        } else if let Some(parent) = parent {
            tree[parent].last_child = previous_sibling;
        }

        if let Some(previous_sibling) = previous_sibling {
            tree[previous_sibling].next_sibling = next_sibling;
        } else if let Some(parent) = parent {
            tree[parent].first_child = next_sibling;
        }
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

impl<T> Index<NodeId> for Tree<T> {
    type Output = Node<T>;

    fn index(&self, node_id: NodeId) -> &Node<T> {
        &self.nodes[node_id.index]
    }
}

impl<T> IndexMut<NodeId> for Tree<T> {
    fn index_mut(&mut self, node_id: NodeId) -> &mut Node<T> {
        &mut self.nodes[node_id.index]
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

         child1_id.set_parent(root_id, &mut tree);
         child2_id.set_parent(root_id, &mut tree);
        // assert!(!result2.is_err(), "it should not return an error");
    }
}
