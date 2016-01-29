use std::cmp::Ordering::*;

pub struct BST<K, V>(NodePtr<K, V>);

struct Node<K, V> {
    key:   K,
    value: V,
    left:  NodePtr<K, V>,
    right: NodePtr<K, V>,
}

type NodePtr<K, V> = Option<Box<Node<K, V>>>;

impl<K, V> BST<K, V> {
    pub fn new() -> Self {
        BST(None)
    }

    pub fn len(&self) -> usize {
        Node::len(&self.0)
    }
}

impl<K: Ord, V> BST<K, V> {
    pub fn find(&self, key: &K) -> Option<&V> {
        Node::find(&self.0, key)
    }

    pub fn find_mut(&mut self, key: &K) -> Option<&mut V> {
        Node::find_mut(&mut self.0, key)
    }
}

impl<K, V> Node<K, V> {
    fn len(ptr: &NodePtr<K, V>) -> usize {
        if let &Some(ref n) = ptr {
            1 + Node::len(&n.left) + Node::len(&n.right)
        } else {0}
    }
}

impl<K: Ord, V> Node<K, V> {
    fn find<'a, 'b>(ptr: &'a NodePtr<K, V>, key: &'b K) -> Option<&'a V> {
        if let &Some(ref n) = ptr {
            match key.cmp(&n.key) {
                Less    => Node::find(&n.left, key),
                Greater => Node::find(&n.right, key),
                Equal   => Some(&n.value),
            }
        } else {None}
    }

    fn find_iter<'a, 'b>(mut ptr: &'a NodePtr<K, V>, key: &'b K) -> Option<&'a V> {
        while let &Some(ref n) = ptr {
            match key.cmp(&n.key) {
                Less    => { ptr = &n.left; }
                Greater => { ptr = &n.right; }
                Equal   => { return Some(&n.value); }
            }
        }

        None
    }

    fn find_mut<'a, 'b>(ptr: &'a mut NodePtr<K, V>, key: &'b K) -> Option<&'a mut V> {
        if let &mut Some(ref mut n) = ptr {
            match key.cmp(&n.key) {
                Less    => Node::find_mut(&mut n.left, key),
                Greater => Node::find_mut(&mut n.right, key),
                Equal   => Some(&mut n.value),
            }
        } else {None}
    }
}

//////////
struct StringPair {
    x : String,
    y : String,
}

fn snd_len(&StringPair {x: ref sx, y: ref sy}: &StringPair) -> usize {
    sx.len() + sy.len()
}
