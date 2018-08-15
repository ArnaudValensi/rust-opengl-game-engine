//! # Vector based tree data structure
//!
//! This vector tree structure is using just a single `Vec` and numerical identifiers (indices in
//! the vector) instead of reference counted pointers like. This means there is no `RefCell` and
//! mutability is handled in a way much more idiomatic to Rust through unique (&mut) access to the
//! vector. The tree can be sent or shared across threads like a `Vec`. This enables general
//! multiprocessing support like parallel tree traversals.
use std::{fmt, mem, cmp};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize,
}

impl NodeId {
    pub fn add_child<T>(self, new_child: NodeId, tree: &mut VecTree<T>) {
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

    pub fn detach<T>(self, tree: &mut VecTree<T>) {
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

    /// Return an iterator of references to this node’s children.
    pub fn children<T>(self, tree: &VecTree<T>) -> Children<T> {
        Children {
            tree,
            node: tree[self].first_child,
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
pub struct VecTree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> VecTree<T> {
    pub fn new() -> VecTree<T> {
        VecTree { nodes: Vec::new() }
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

impl<T> Index<NodeId> for VecTree<T> {
    type Output = Node<T>;

    fn index(&self, node_id: NodeId) -> &Node<T> {
        &self.nodes[node_id.index]
    }
}

impl<T> IndexMut<NodeId> for VecTree<T> {
    fn index_mut(&mut self, node_id: NodeId) -> &mut Node<T> {
        &mut self.nodes[node_id.index]
    }
}

/// An iterator of references to the children of a given node.
pub struct Children<'a, T: 'a> {
    tree: &'a VecTree<T>,
    node: Option<NodeId>,
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        match self.node.take() {
            Some(node) => {
                self.node = self.tree[node].next_sibling;
                Some(node)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VecTree;

    #[test]
    fn create_tree() {
        let mut tree = VecTree::new();

        let root_node = tree.new_node(1);
        let child_node_1 = tree.new_node(2);
        let child_node_2 = tree.new_node(3);

        root_node.add_child(child_node_1, &mut tree);
        root_node.add_child(child_node_2, &mut tree);

        assert!(tree.nodes.len() == 3, "it should have 3 nodes in the tree");
    }

    #[test]
    fn iterate_over_children() {
        let mut tree = VecTree::new();

        let root_node = tree.new_node(1);
        let child_node_1 = tree.new_node(2);
        let child_node_2 = tree.new_node(3);
        let child_node_3 = tree.new_node(4);
        let grandchild = tree.new_node(5);

        root_node.add_child(child_node_1, &mut tree);
        root_node.add_child(child_node_2, &mut tree);
        root_node.add_child(child_node_3, &mut tree);
        child_node_3.add_child(grandchild, &mut tree);

        assert_eq!(
            root_node.children(&tree).map(|node| tree[node].data).collect::<Vec<_>>(),
            [2, 3, 4]
        );

        assert_eq!(
            child_node_1.children(&tree).map(|node| tree[node].data).collect::<Vec<_>>(),
            []
        );

        assert_eq!(
            child_node_2.children(&tree).map(|node| tree[node].data).collect::<Vec<_>>(),
            []
        );

        assert_eq!(
            child_node_3.children(&tree).map(|node| tree[node].data).collect::<Vec<_>>(),
            [5]
        );
    }
}
