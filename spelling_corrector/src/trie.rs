use std::ops::Index;

/// A trie has a root node, knows its branching factor, and has a field
/// containing `None` that can be returned as a borrowed reference.
#[derive(Debug)]
pub struct TrieMap<T> {
    node:   Box<Node<T>>,
    factor: usize,
    none:   Option<T>,
}

/// A trie node may have a value, and has a vector of nullable pointers to
/// child nodes.
#[derive(Debug)]
struct Node<T> {
    value:    Option<T>,
    children: Vec<Option<Box<Node<T>>>>,
}

/// A cursor marks a position in the trie and can be used to traverse it.
#[derive(Copy, Clone)]
pub struct Cursor<'a, T: 'a> {
    node: &'a Node<T>,
}

/// A mutable cursor marks a position in the trie, and can be used to
/// traverse and modify it.
pub struct CursorMut<'a, T: 'a> {
    node:   &'a mut Node<T>,
    factor: usize,
}

impl<T> Node<T> {
    /// Creates a new node with the given branching factor.
    fn new(factor: usize) -> Self {
        let mut node = Node {
            value:    None,
            children: Vec::with_capacity(factor),
        };

        for _ in 0 .. factor {
            node.children.push(None);
        }

        node
    }

    /// Creates and boxes a new node with the given branching factor.
    fn boxed(factor: usize) -> Box<Self> {
        Box::new(Self::new(factor))
    }

    /// Converts this node into a cursor.
    fn cursor(&self) -> Cursor<T> {
        Cursor { node: self }
    }

    /// Converts this mutable node into a mutable cursor with the given
    /// branching factor. (The branching factor is needed in order to
    /// create new nodes.
    fn cursor_mut(&mut self, factor: usize) -> CursorMut<T> {
        CursorMut {
            node: &mut *self,
            factor: factor,
        }
    }

    fn lookup<'a, 'b>(&'a self, key: &'b [usize], default: &'a Option<T>)
        -> &'a Option<T>
    {
        let mut cursor = self.cursor();

        for &each in key.iter() {
            if !cursor.descend(each) { return default }
        }

        cursor.value()
    }
}

impl<'a, T> Cursor<'a, T> {
    pub fn value(&self) -> &'a Option<T> {
        &self.node.value
    }

    pub fn child(&self, key: usize) -> Option<Self> {
        self.node.children[key].as_ref().map(|n| n.cursor())
    }

    pub fn descend(&mut self, key: usize) -> bool {
        match self.child(key) {
            None         => false,
            Some(cursor) => { *self = cursor; true },
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn freeze(&self) -> Cursor<T> {
        self.node.cursor()
    }

    pub fn value(&mut self) -> &mut Option<T> {
        &mut self.node.value
    }

    pub fn child(&'a mut self, key: usize) -> Option<Self> {
        let factor = self.factor;
        self.node.children[key].as_mut().map(|n| n.cursor_mut(factor))
    }

    pub fn into_child(self, key: usize) -> Option<Self> {
        let factor = self.factor;
        self.node.children[key].as_mut().map(|n| n.cursor_mut(factor))
    }

    pub fn child_add(&'a mut self, key: usize) -> Self {
        match &mut self.node.children[key] {
            &mut Some(ref mut child) => child,
            otherwise => {
                *otherwise = Some(Node::boxed(self.factor));
                otherwise.as_mut().unwrap()
            },
        }
        .cursor_mut(self.factor)
    }

    pub fn into_child_add(self, key: usize) -> Self {
        match &mut self.node.children[key] {
            &mut Some(ref mut child) => child,
            otherwise => {
                *otherwise = Some(Node::boxed(self.factor));
                otherwise.as_mut().unwrap()
            },
        }
        .cursor_mut(self.factor)
    }
}

impl<T> TrieMap<T> {
    pub fn new(factor: usize) -> Self {
        TrieMap {
            node:   Node::boxed(factor),
            factor: factor,
            none:   None,
        }
    }

    pub fn cursor(&self) -> Cursor<T> {
        self.node.cursor()
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        self.node.cursor_mut(self.factor)
    }

    pub fn contains(&self, key: &[usize]) -> bool {
        self[key].is_some()
    }
}

impl<'b, T> Index<&'b [usize]> for TrieMap<T> {
    type Output = Option<T>;

    fn index(&self, key: &[usize]) -> &Option<T> {
        self.node.lookup(key, &self.none)
    }
}

