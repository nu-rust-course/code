use std::cmp::Ordering::*;
use std::mem;

#[derive(Debug)]
pub struct BST<K, V>(Link<K, V>);

#[derive(Debug)]
struct Node<K, V> {
    key:   K,
    value: V,
    left:  Link<K, V>,
    right: Link<K, V>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct CursorMut<'a, K: 'a, V: 'a>(Option<&'a mut Node<K, V>>);

impl<K, V> BST<K, V> {
    pub fn new() -> Self {
        BST(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn len(&self) -> usize {
        Node::len_iter(&self.0)
    }
}

impl<K, V> Default for BST<K, V> {
    fn default() -> Self {
        BST::new()
    }
}

impl<K: Ord, V> BST<K, V> {
    pub fn find(&self, key: &K) -> Option<&V> {
        Node::find_iter(&self.0, key) }

    pub fn find_mut(&mut self, key: &K) -> Option<&mut V> {
        Node::find_mut_iter(&mut self.0, key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)> {
        Node::insert_rec(&mut self.0, key, value)
    }
}

impl<K, V> Node<K, V> {
    #[allow(dead_code)]
    fn len_rec(ptr: &Link<K, V>) -> usize {
        if let Some(ref node_ptr) = *ptr {
            1 + Node::len_rec(&node_ptr.left) + Node::len_rec(&node_ptr.right)
        } else {0}
    }

    fn len_iter(ptr: &Link<K, V>) -> usize {
        let mut result = 0;
        let mut stack = vec![ptr];

        while let Some(each) = stack.pop() {
            if let Some(ref node_ptr) = *each {
                result += 1;
                stack.push(&node_ptr.left);
                stack.push(&node_ptr.right);
            }
        }

        result
    }
}

impl<K: Ord, V> Node<K, V> {
    #[allow(dead_code)]
    fn find_rec<'a, 'b>(ptr: &'a Link<K, V>, key: &'b K) -> Option<&'a V> {
        if let Some(ref n) = *ptr {
            match key.cmp(&n.key) {
                Less    => Node::find_rec(&n.left, key),
                Greater => Node::find_rec(&n.right, key),
                Equal   => Some(&n.value),
            }
        } else {None}
    }

    fn find_iter<'a, 'b>(mut ptr: &'a Link<K, V>, key: &'b K)
        -> Option<&'a V>
    {
        while let Some(ref n) = *ptr {
            match key.cmp(&n.key) {
                Less    => { ptr = &n.left; }
                Greater => { ptr = &n.right; }
                Equal   => { return Some(&n.value); }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn find_mut_rec<'a, 'b>(ptr: &'a mut Link<K, V>, key: &'b K)
        -> Option<&'a mut V>
    {
        if let Some(ref mut n) = *ptr {
            match key.cmp(&n.key) {
                Less    => Node::find_mut_rec(&mut n.left, key),
                Greater => Node::find_mut_rec(&mut n.right, key),
                Equal   => Some(&mut n.value),
            }
        } else {None}
    }

    fn find_mut_iter<'a, 'b>(ptr: &'a mut Link<K, V>, key: &'b K)
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

    fn insert_rec(ptr: &mut Link<K, V>, key: K, value: V) -> Option<(K, V)> {
        match *ptr {
            None => {
                *ptr = Some(Box::new(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                }));
                None
            }

            Some(ref mut node_ptr) => {
                match key.cmp(&node_ptr.key) {
                    Less    => Node::insert_rec(&mut node_ptr.left, key, value),
                    Greater => Node::insert_rec(&mut node_ptr.right, key, value),
                    Equal   =>
                        Some((mem::replace(&mut node_ptr.key, key),
                              mem::replace(&mut node_ptr.value, value))),
                }
            }
        }
    }

    // Challenge: insert_iter
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

    fn key(&self) -> Option<&K> {
        self.0.as_ref().map(|n| &n.key)
    }

    fn into_value(self) -> Option<&'a mut V> {
        self.0.map(|n| &mut n.value)
    }
}

#[test]
fn bst_test() {
    let mut bst = BST::new();
    assert_eq!( bst.insert("one", 1), None );
    assert_eq!( bst.insert("two", 2), None );
    assert_eq!( bst.insert("three", 3), None );

    assert_eq!( bst.find(&"one"), Some(&1) );
    assert_eq!( bst.find(&"two"), Some(&2) );
    assert_eq!( bst.find(&"three"), Some(&3) );
    assert_eq!( bst.find(&"four"), None );

    *bst.find_mut(&"three").unwrap() = 4;

    assert_eq!( bst.find(&"three"), Some(&4) );

    assert_eq!( bst.insert("four", 7), None );
    assert_eq!( bst.insert("four", 8), Some(("four", 7)) );
}
