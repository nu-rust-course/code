//! Sets, represented as sorted, singly-linked lists.

use std::cmp::Ordering::*;

#[derive(Debug)]
pub struct Set<T> {
    head: Link<T>,
    len:  usize,
}
// INVARIANT: the elements are sorted in ascending order

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    link: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T, link: Link<T>) -> Option<Box<Self>> {
        Some(Box::new(Node { data, link, }))
    }
}

impl<T> Set<T> {
    pub fn new() -> Self {
        Set {
            len: 0,
            head: None,
        }
    }
}

impl<T: Ord> Set<T> {
    pub fn contains(&self, element: &T) -> bool {
        let mut current = &self.head;

        while let Some(box_node) = current {
            match element.cmp(&box_node.data) {
                Less => return false,
                Equal => return true,
                Greater => current = &box_node.link,
            }
        }

        false
    }

    pub fn insert(&mut self, element: T) -> Option<T> {

        unimplemented!()
    }
}

#[derive(Debug)]
struct CursorMut<'a, T> {
    link: Option<&'a mut Link<T>>,
    len:  &'a mut usize,
}

impl<'a, T> CursorMut<'a, T> {
    fn new(set: &'a mut Set<T>) -> Self {
        CursorMut {
            link: Some(&mut set.head),
            len:  &mut set.len,
        }
    }

    fn is_empty(&self) -> bool {
        self.link.as_ref().expect("bad").is_none()
    }

    fn data(&self) -> Option<&T> {
        self.link.as_ref().expect("bad")
            .as_ref().map(|node_ptr| &node_ptr.data)
    }

    fn data_mut(&mut self) -> Option<&mut T> {
        self.link.as_mut().expect("bad")
            .as_mut().map(|node_ptr| &mut node_ptr.data)
    }

    fn advance(&mut self) {
        let taken = self.link.take();
        let expected = taken.expect("bad");
        let mapped = expected.map(|node_ptr| &mut node_ptr.link);
        self.link = mapped;
    }

    fn insert(&mut self, data: T) {
        let link_ptr =
            self.link.as_mut().expect("bad");
        **link_ptr = Node::new(data, link_ptr.take());
        *self.len += 1;
    }

}









