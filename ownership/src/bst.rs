use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::mem;

pub struct BST<K, V>(NodePtr<K, V>);

struct Node<K, V> {
    key:   K,
    value: V,
    left:  NodePtr<K, V>,
    right: NodePtr<K, V>,
}

type NodePtr<K, V> = Option<Box<Node<K, V>>>;

struct CursorMut<'a, K: 'a, V: 'a>(Option<&'a mut Node<K, V>>);

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
        Node::find_iter(&self.0, key)
    }

    pub fn find_mut(&mut self, key: &K) -> Option<&mut V> {
        Node::find_mut_rec(&mut self.0, key)
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
    fn find_rec<'a, 'b>(ptr: &'a NodePtr<K, V>, key: &'b K) -> Option<&'a V> {
        if let &Some(ref n) = ptr {
            match key.cmp(&n.key) {
                Less    => Node::find_rec(&n.left, key),
                Greater => Node::find_rec(&n.right, key),
                Equal   => Some(&n.value),
            }
        } else {None}
    }

    fn find_iter<'a, 'b>(mut ptr: &'a NodePtr<K, V>, key: &'b K)
        -> Option<&'a V>
    {
        while let &Some(ref n) = ptr {
            match key.cmp(&n.key) {
                Less    => { ptr = &n.left; }
                Greater => { ptr = &n.right; }
                Equal   => { return Some(&n.value); }
            }
        }

        None
    }

    fn find_mut_rec<'a, 'b>(ptr: &'a mut NodePtr<K, V>, key: &'b K)
        -> Option<&'a mut V>
    {
        if let &mut Some(ref mut n) = ptr {
            match key.cmp(&n.key) {
                Less    => Node::find_mut_rec(&mut n.left, key),
                Greater => Node::find_mut_rec(&mut n.right, key),
                Equal   => Some(&mut n.value),
            }
        } else {None}
    }

    fn find_mut_iter<'a, 'b>(ptr: &'a mut NodePtr<K, V>, key: &'b K)
        -> Option<&'a mut V>
    {
        let mut cur: CursorMut<'a, K, V> =
            CursorMut(ptr.as_mut().map(|node| &mut **node));

        loop {
            match cur.key().map(|k| key.cmp(k)) {
                Some(Less)     => { cur.left(); }
                Some(Greater)  => { cur.right(); }
                Some(Equal)    => { return cur.into_value(); }
                None           => { return None; }
            }
        }
    }
}

impl<'a, K, V> CursorMut<'a, K, V> {
    fn left(&mut self) {
        self.0.take().map(|node| {
            self.0 = node.left.as_mut().map(|node| &mut **node);
        });
    }

    fn right(&mut self) {
        self.0.take().map(|node| {
            self.0 = node.right.as_mut().map(|node| &mut **node);
        });
    }

    fn key(&mut self) -> Option<&K> {
        self.0.as_ref().map(|n| &n.key)
    }

    fn into_value(self) -> Option<&'a mut V> {
        self.0.map(|n| &mut n.value)
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
