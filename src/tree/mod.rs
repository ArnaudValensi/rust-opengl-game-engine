use std::{fmt, mem, cmp};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize,
}

impl NodeId {
    pub fn add_child<T>(self, new_child: NodeId, tree: &mut Tree<T>) {
        new_child.detach(tree);
        let last_child_opt;
        {
            let (self_borrow, new_child_borrow) =
                tree
                    .nodes
                    .get_pair_mut(self.index, new_child.index, "Can not append a node to itself");
            new_child_borrow.parent = Some(self);
            last_child_opt = mem::replace(&mut self_borrow.last_child, Some(new_child));
            if let Some(last_child) = last_child_opt {
                new_child_borrow.previous_sibling = Some(last_child);
            } else {
                debug_assert!(self_borrow.first_child.is_none());
                self_borrow.first_child = Some(new_child);
            }
        }
        if let Some(last_child) = last_child_opt {
            debug_assert!(tree[last_child].next_sibling.is_none());
            tree[last_child].next_sibling = Some(new_child);
        }
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

// TODO: Change name "Tree" to "Pool"?
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

trait GetPairMut<T> {
    /// Get mutable references to two distinct nodes. Panics if the two given IDs are the same.
    fn get_pair_mut(&mut self, a: usize, b: usize, same_index_error_message: &'static str) -> (&mut T, &mut T);
}

impl<T> GetPairMut<T> for Vec<T> {
    fn get_pair_mut(&mut self, a: usize, b: usize, same_index_error_message: &'static str) -> (&mut T, &mut T) {
        if a == b {
            panic!(same_index_error_message)
        }
        let (xs, ys) = self.split_at_mut(cmp::max(a, b));
        if a < b {
            (&mut xs[a], &mut ys[0])
        } else {
            (&mut ys[0], &mut xs[b])
        }
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

        let root_node = tree.new_node(1);
        let child_node_1 = tree.new_node(2);
        let child_node_2 = tree.new_node(3);

        root_node.add_child(child_node_1, &mut tree);
        root_node.add_child(child_node_2, &mut tree);

        assert!(tree.nodes.len() == 3, "it should have 3 nodes in the tree");
    }
}
