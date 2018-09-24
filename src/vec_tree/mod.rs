//! # Vector based tree data structure
//!
//! This vector tree structure is using just a single `Vec` and numerical identifiers (indices in
//! the vector) instead of reference counted pointers like. This means there is no `RefCell` and
//! mutability is handled in a way much more idiomatic to Rust through unique (&mut) access to the
//! vector. The tree can be sent or shared across threads like a `Vec`. This enables general
//! multiprocessing support like parallel tree traversals.
use std::ops::{Index, IndexMut};
use std::{cmp, fmt, mem};

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize,
}

impl NodeId {
    /// Append a new child to this node, after existing children.
    pub fn append_child<T>(self, new_child: NodeId, tree: &mut VecTree<T>) {
        new_child.detach(tree);

        let last_child_opt;
        {
            let (self_borrow, new_child_borrow) = tree.nodes.get_pair_mut(
                self.index,
                new_child.index,
                "Can not append a node to itself",
            );
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

    /// Prepend a new child to this node, before existing children.
    pub fn prepend_child<T>(self, new_child: NodeId, tree: &mut VecTree<T>) {
        new_child.detach(tree);
        let first_child_opt;
        {
            let (self_borrow, new_child_borrow) = tree.nodes.get_pair_mut(
                self.index,
                new_child.index,
                "Can not prepend a node to itself",
            );
            new_child_borrow.parent = Some(self);
            first_child_opt = mem::replace(&mut self_borrow.first_child, Some(new_child));
            if let Some(first_child) = first_child_opt {
                new_child_borrow.next_sibling = Some(first_child);
            } else {
                self_borrow.last_child = Some(new_child);
                debug_assert!(&self_borrow.first_child.is_none());
            }
        }
        if let Some(first_child) = first_child_opt {
            debug_assert!(tree[first_child].previous_sibling.is_none());
            tree[first_child].previous_sibling = Some(new_child);
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

    /// Insert a new sibling after this node.
    pub fn insert_after<T>(self, new_sibling: NodeId, tree: &mut VecTree<T>) {
        new_sibling.detach(tree);
        let next_sibling_opt;
        let parent_opt;
        {
            let (self_borrow, new_sibling_borrow) = tree.nodes.get_pair_mut(
                self.index,
                new_sibling.index,
                "Can not insert a node after itself",
            );
            parent_opt = self_borrow.parent;
            new_sibling_borrow.parent = parent_opt;
            new_sibling_borrow.previous_sibling = Some(self);
            next_sibling_opt = mem::replace(&mut self_borrow.next_sibling, Some(new_sibling));
            if let Some(next_sibling) = next_sibling_opt {
                new_sibling_borrow.next_sibling = Some(next_sibling);
            }
        }

        if let Some(next_sibling) = next_sibling_opt {
            debug_assert!(tree[next_sibling].previous_sibling.unwrap().index == self.index);
            tree[next_sibling].previous_sibling = Some(new_sibling);
        } else if let Some(parent) = parent_opt {
            debug_assert!(tree[parent].last_child.unwrap().index == self.index);
            tree[parent].last_child = Some(new_sibling);
        }
    }

    /// Insert a new sibling before this node.
    pub fn insert_before<T>(self, new_sibling: NodeId, tree: &mut VecTree<T>) {
        new_sibling.detach(tree);
        let previous_sibling_opt;
        let parent_opt;
        {
            let (self_borrow, new_sibling_borrow) = tree.nodes.get_pair_mut(
                self.index,
                new_sibling.index,
                "Can not insert a node before itself",
            );
            parent_opt = self_borrow.parent;
            new_sibling_borrow.parent = parent_opt;
            new_sibling_borrow.next_sibling = Some(self);
            previous_sibling_opt =
                mem::replace(&mut self_borrow.previous_sibling, Some(new_sibling));
            if let Some(previous_sibling) = previous_sibling_opt {
                new_sibling_borrow.previous_sibling = Some(previous_sibling);
            }
        }
        if let Some(previous_sibling) = previous_sibling_opt {
            debug_assert!(tree[previous_sibling].next_sibling.unwrap().index == self.index);
            tree[previous_sibling].next_sibling = Some(new_sibling);
        } else if let Some(parent) = parent_opt {
            debug_assert!(tree[parent].first_child.unwrap().index == self.index);
            tree[parent].first_child = Some(new_sibling);
        }
    }

    /// Return an iterator of references to this node’s children.
    pub fn children<T>(self, tree: &VecTree<T>) -> Children<T> {
        Children {
            tree,
            node: tree[self].first_child,
        }
    }

    /// Return an iterator of references to this node and its descendants, in tree order.
    fn traverse<T>(self, tree: &VecTree<T>) -> Traverse<T> {
        Traverse {
            tree,
            root: self,
            next: Some(NodeEdge::Start(self)),
        }
    }

    /// Return an iterator of references to this node and its descendants, with deoth in the tree,
    /// in tree order.
    fn traverse_with_depth<T>(self, tree: &VecTree<T>) -> TraverseWithDepth<T> {
        TraverseWithDepth {
            tree,
            root: self,
            next: Some(NodeEdgeWithDepth::Start(self, 0)),
        }
    }

    /// Return an iterator of references to this node and its descendants, in tree order.
    ///
    /// Parent nodes appear before the descendants.
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn descendants<T>(self, tree: &VecTree<T>) -> Descendants<T> {
        Descendants(self.traverse(tree))
    }

    /// Return an iterator of references to this node and its descendants, with deoth in the tree,
    /// in tree order.
    ///
    /// Parent nodes appear before the descendants.
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn descendants_with_depth<T>(self, tree: &VecTree<T>) -> DescendantsWithDepth<T> {
        DescendantsWithDepth(self.traverse_with_depth(tree))
    }

    /// Return an iterator of references to this node and the siblings before it.
    ///
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn preceding_siblings<T>(self, tree: &VecTree<T>) -> PrecedingSiblings<T> {
        PrecedingSiblings {
            tree,
            node: Some(self),
        }
    }

    /// Return an iterator of references to this node and the siblings after it.
    ///
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn following_siblings<T>(self, tree: &VecTree<T>) -> FollowingSiblings<T> {
        FollowingSiblings {
            tree,
            node: Some(self),
        }
    }

    /// Return an iterator of references to this node and its ancestors.
    ///
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn ancestors<T>(self, tree: &VecTree<T>) -> Ancestors<T> {
        Ancestors {
            tree,
            node: Some(self),
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

    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}

trait GetPairMut<T> {
    /// Get mutable references to two distinct nodes. Panics if the two given IDs are the same.
    fn get_pair_mut(
        &mut self,
        a: usize,
        b: usize,
        same_index_error_message: &'static str,
    ) -> (&mut T, &mut T);
}

impl<T> GetPairMut<T> for Vec<T> {
    fn get_pair_mut(
        &mut self,
        a: usize,
        b: usize,
        same_index_error_message: &'static str,
    ) -> (&mut T, &mut T) {
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

macro_rules! impl_node_iterator {
    ($name:ident, $next:expr) => {
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = NodeId;

            fn next(&mut self) -> Option<NodeId> {
                match self.node.take() {
                    Some(node) => {
                        self.node = $next(&self.tree[node]);
                        Some(node)
                    }
                    None => None,
                }
            }
        }
    };
}

/// An iterator of references to the children of a given node.
pub struct Children<'a, T: 'a> {
    tree: &'a VecTree<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(Children, |node: &Node<T>| node.next_sibling);

/// An iterator of references to the siblings before a given node.
pub struct PrecedingSiblings<'a, T: 'a> {
    tree: &'a VecTree<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(PrecedingSiblings, |node: &Node<T>| node.previous_sibling);

/// An iterator of references to the siblings after a given node.
pub struct FollowingSiblings<'a, T: 'a> {
    tree: &'a VecTree<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(FollowingSiblings, |node: &Node<T>| node.next_sibling);

/// An iterator of references to the ancestors a given node.
pub struct Ancestors<'a, T: 'a> {
    tree: &'a VecTree<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(Ancestors, |node: &Node<T>| node.parent);

#[derive(Debug, Clone)]
/// Indicator if the node is at a start or endpoint of the tree
pub enum NodeEdge<T> {
    /// Indicates that start of a node that has children. Yielded by `Traverse::next` before the
    /// node’s descendants.
    Start(T),

    /// Indicates that end of a node that has children. Yielded by `Traverse::next` after the
    /// node’s descendants.
    End(T),
}

/// An iterator of references to a given node and its descendants, in depth-first search pre-order
/// NLR traversal.
/// https://en.wikipedia.org/wiki/Tree_traversal#Pre-order_(NLR)
pub struct Traverse<'a, T: 'a> {
    tree: &'a VecTree<T>,
    root: NodeId,
    next: Option<NodeEdge<NodeId>>,
}

impl<'a, T> Iterator for Traverse<'a, T> {
    type Item = NodeEdge<NodeId>;

    fn next(&mut self) -> Option<NodeEdge<NodeId>> {
        match self.next.take() {
            Some(item) => {
                self.next = match item {
                    NodeEdge::Start(node) => match self.tree[node].first_child {
                        Some(first_child) => Some(NodeEdge::Start(first_child)),
                        None => Some(NodeEdge::End(node)),
                    },
                    NodeEdge::End(node) => {
                        if node.index == self.root.index {
                            None
                        } else {
                            match self.tree[node].next_sibling {
                                Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                                None => {
                                    match self.tree[node].parent {
                                        Some(parent) => Some(NodeEdge::End(parent)),

                                        // `node.parent()` here can only be `None`
                                        // if the tree has been modified during iteration,
                                        // but silently stoping iteration
                                        // seems a more sensible behavior than panicking.
                                        None => None,
                                    }
                                }
                            }
                        }
                    }
                };
                Some(item)
            }
            None => None,
        }
    }
}

/// An iterator of references to a given node and its descendants, in tree order.
pub struct Descendants<'a, T: 'a>(pub Traverse<'a, T>);

impl<'a, T> Iterator for Descendants<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        loop {
            match self.0.next() {
                Some(NodeEdge::Start(node)) => return Some(node),
                Some(NodeEdge::End(_)) => {}
                None => return None,
            }
        }
    }
}

#[derive(Debug, Clone)]
/// Indicator if the node is at a start or endpoint of the tree
pub enum NodeEdgeWithDepth<T> {
    /// Indicates that start of a node that has children. Yielded by `Traverse::next` before the
    /// node’s descendants.
    Start(T, u32),

    /// Indicates that end of a node that has children. Yielded by `Traverse::next` after the
    /// node’s descendants.
    End(T, u32),
}

/// An iterator of references to a given node and its descendants, with depth, in depth-first
/// search pre-order NLR traversal.
/// https://en.wikipedia.org/wiki/Tree_traversal#Pre-order_(NLR)
pub struct TraverseWithDepth<'a, T: 'a> {
    tree: &'a VecTree<T>,
    root: NodeId,
    next: Option<NodeEdgeWithDepth<NodeId>>,
}

impl<'a, T> Iterator for TraverseWithDepth<'a, T> {
    type Item = NodeEdgeWithDepth<NodeId>;

    fn next(&mut self) -> Option<NodeEdgeWithDepth<NodeId>> {
        match self.next.take() {
            Some(item) => {
                self.next = match item {
                    NodeEdgeWithDepth::Start(node, depth) => match self.tree[node].first_child {
                        Some(first_child) => Some(NodeEdgeWithDepth::Start(first_child, depth + 1)),
                        None => Some(NodeEdgeWithDepth::End(node, depth)),
                    },
                    NodeEdgeWithDepth::End(node, depth) => {
                        if node.index == self.root.index {
                            None
                        } else {
                            match self.tree[node].next_sibling {
                                Some(next_sibling) => {
                                    Some(NodeEdgeWithDepth::Start(next_sibling, depth))
                                }
                                None => {
                                    match self.tree[node].parent {
                                        Some(parent) => {
                                            Some(NodeEdgeWithDepth::End(parent, depth - 1))
                                        }

                                        // `node.parent()` here can only be `None`
                                        // if the tree has been modified during iteration,
                                        // but silently stoping iteration
                                        // seems a more sensible behavior than panicking.
                                        None => None,
                                    }
                                }
                            }
                        }
                    }
                };
                Some(item)
            }
            None => None,
        }
    }
}

/// An iterator of references to a given node and its descendants, with depth, in tree order.
pub struct DescendantsWithDepth<'a, T: 'a>(pub TraverseWithDepth<'a, T>);

impl<'a, T> Iterator for DescendantsWithDepth<'a, T> {
    type Item = (NodeId, u32);

    fn next(&mut self) -> Option<(NodeId, u32)> {
        loop {
            match self.0.next() {
                Some(NodeEdgeWithDepth::Start(node, depth)) => return Some((node, depth)),
                Some(NodeEdgeWithDepth::End(_, _)) => {}
                None => return None,
            }
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

        root_node.append_child(child_node_1, &mut tree);
        root_node.append_child(child_node_2, &mut tree);

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

        root_node.append_child(child_node_1, &mut tree);
        root_node.append_child(child_node_2, &mut tree);
        root_node.append_child(child_node_3, &mut tree);
        child_node_3.append_child(grandchild, &mut tree);

        assert_eq!(
            root_node
                .children(&tree)
                .map(|node| tree[node].data)
                .collect::<Vec<_>>(),
            [2, 3, 4]
        );

        assert_eq!(
            child_node_1
                .children(&tree)
                .map(|node| tree[node].data)
                .collect::<Vec<_>>(),
            []
        );

        assert_eq!(
            child_node_2
                .children(&tree)
                .map(|node| tree[node].data)
                .collect::<Vec<_>>(),
            []
        );

        assert_eq!(
            child_node_3
                .children(&tree)
                .map(|node| tree[node].data)
                .collect::<Vec<_>>(),
            [5]
        );
    }

    #[test]
    fn iterate_over_descendants_with_depth() {
        let mut tree = VecTree::new();

        // 0
        // 1----2--3
        // |    |
        // |    7
        // 4--5
        // |
        // 6
        let root_node = tree.new_node(0);
        let node_1 = tree.new_node(1);
        let node_2 = tree.new_node(2);
        let node_3 = tree.new_node(3);
        let node_4 = tree.new_node(4);
        let node_5 = tree.new_node(5);
        let node_6 = tree.new_node(6);
        let node_7 = tree.new_node(7);

        root_node.append_child(node_1, &mut tree);
        root_node.append_child(node_2, &mut tree);
        root_node.append_child(node_3, &mut tree);
        node_1.append_child(node_4, &mut tree);
        node_1.append_child(node_5, &mut tree);
        node_4.append_child(node_6, &mut tree);
        node_2.append_child(node_7, &mut tree);

        let descendants = root_node
            .descendants_with_depth(&tree)
            .map(|(node, depth)| (tree[node].data, depth))
            .collect::<Vec<(i32, u32)>>();

        let expected_result = [
            (0, 0),
            (1, 1),
            (4, 2),
            (6, 3),
            (5, 2),
            (2, 1),
            (7, 2),
            (3, 1),
        ];

        assert_eq!(descendants, expected_result);
    }
}
