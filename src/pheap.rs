use std::{collections::VecDeque, ptr::NonNull};

/// Min-heap data structure
#[derive(Debug)]
pub struct PairingHeap<V, P> {
    root: Option<NonNull<Node<V, P>>>,
    len: usize,
}

impl<V, P> Default for PairingHeap<V, P> {
    fn default() -> Self {
        Self { root: None, len: 0 }
    }
}

impl<V, P> PairingHeap<V, P> {
    /// Creates an empty pairing heap.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of elements stored in the heap.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks whether the heap is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the minimum element, which is the root element, of the heap.
    #[inline]
    pub fn find_min(&self) -> Option<V>
    where
        V: Clone,
    {
        unsafe { self.root.map(|node| node.as_ref().key.clone()) }
    }

    /// Returns the minimum element, which is the root element, and its priority in a tuple of the heap.
    #[inline]
    pub fn find_min_with_prio(&self) -> Option<(V, P)>
    where
        V: Clone,
        P: Clone,
    {
        match self.root {
            Some(node) => unsafe {
                let r = node.as_ref();
                Some((r.key.clone(), r.prio.clone()))
            },
            None => None,
        }
    }

    /// Merges two heaps together and forms a new heap.
    ///
    /// If one heap is empty, the other heap will be returned and vice versa. Otherwise, [...]
    #[inline]
    pub fn merge(self, other: Self) -> Self
    where
        P: PartialOrd,
    {
        let len = self.len() + other.len();
        let root = Self::merge_nodes(self.root, other.root);

        Self { root, len }
    }

    #[inline]
    fn merge_nodes(
        node1: Option<NonNull<Node<V, P>>>,
        node2: Option<NonNull<Node<V, P>>>,
    ) -> Option<NonNull<Node<V, P>>>
    where
        P: PartialOrd,
    {
        match (node1, node2) {
            (Some(root1), Some(root2)) => unsafe {
                let root = if root1.as_ref().prio < root2.as_ref().prio {
                    Self::meld(root1, root2)
                } else {
                    Self::meld(root2, root1)
                };
                Some(root)
            },
            (Some(_), None) => node1,
            (None, Some(_)) => node2,
            _ => node1,
        }
    }

    #[inline(always)]
    unsafe fn meld(node1: NonNull<Node<V, P>>, node2: NonNull<Node<V, P>>) -> NonNull<Node<V, P>> {
        (*node2.as_ptr()).parent = Some(node1);
        (*node2.as_ptr()).right = node1.as_ref().left;
        (*node1.as_ptr()).left = Some(node2);
        node1
    }

    /// Inserts a new element to the heap.
    #[inline]
    pub fn insert(&mut self, key: V, prio: P)
    where
        P: PartialOrd,
    {
        let node = NonNull::new(Box::leak(Box::new(Node::new(key, prio))));
        self.root = Self::merge_nodes(self.root, node);
        self.len += 1;
    }

    /// Deletes the minimum element, which is the root, of the heap.
    pub fn delete_min(&mut self)
    where
        P: PartialOrd,
    {
        if let Some(root) = self.root {
            unsafe {
                self.len -= 1;
                let mut targ = (*root.as_ptr()).left.take();
                if targ.is_none() {
                    self.root = None;
                    return;
                }

                // TODO: optimise so that capacity is known here.
                let mut tmp_nodes = VecDeque::new();

                // First pass: left to right
                while let Some(node) = targ {
                    (*node.as_ptr()).parent = None;
                    let right = (*node.as_ptr()).right.take();

                    let node_next = match right {
                        Some(node_right) => {
                            let next = (*node_right.as_ptr()).right.take();
                            (*node_right.as_ptr()).parent = None;
                            next
                        }
                        None => None,
                    };

                    tmp_nodes.push_back(Self::merge_nodes(Some(node), right));

                    targ = node_next;
                }

                // Second pass: right to left
                // If left is not None, there must be at least one element in VecDeque.
                // So unwrap() is safe here.
                let mut node = tmp_nodes.pop_back().unwrap();

                while let Some(node_prev) = tmp_nodes.pop_back() {
                    node = Self::merge_nodes(node, node_prev);
                }

                self.root = node
            }
        }
    }
}

#[derive(Debug)]
struct Node<K, P> {
    key: K,
    prio: P,
    /// Pointer to a node's parent.
    parent: Option<NonNull<Node<K, P>>>,
    /// Pointer to a node's first (or left-most) child.
    left: Option<NonNull<Node<K, P>>>,
    /// Pointer to a node's next older sibling.
    right: Option<NonNull<Node<K, P>>>,
}

impl<K, P> Node<K, P> {
    #[inline]
    pub fn new(key: K, prio: P) -> Self {
        Self {
            key,
            prio,
            parent: None,
            left: None,
            right: None,
        }
    }
}
