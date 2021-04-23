mod index_graph;

pub use index_graph::IndexGraph;

use std::collections::HashSet;

/// Make edges abstract to enforce a representation invariant that makes
/// each edge representation unique. In particular, we want that
/// `Edge::new(u, v) == Edge::new(v, u)`.
mod edge {
    pub type Vertex = usize;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// Graph edge with invariant that `u <= v`.
    pub struct Edge {
        u: Vertex,
        v: Vertex,
    }

    impl Edge {
        pub fn new(mut u: usize, mut v: usize) -> Self {
            if v < u {
                std::mem::swap(&mut u, &mut v);
            }
            Edge { u, v }
        }
    }
}

use edge::{Edge, Vertex};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeSetGraph {
    len: usize,
    edges: HashSet<Edge>,
}

impl EdgeSetGraph {
    pub fn new() -> Self {
        Self::with_vertices(0)
    }

    pub fn with_vertices(len: usize) -> Self {
        Self {
            len,
            edges: HashSet::new(),
        }
    }

    pub fn from_edges<I>(edges: I) -> Self
    where
        I: IntoIterator<Item = (Vertex, Vertex)>,
    {
        let mut len = 0;
        let edges = edges
            .into_iter()
            .map(|(u, v)| {
                len = len.max(u + 1).max(v + 1);
                Edge::new(u, v)
            })
            .collect();

        Self { len, edges }
    }

    /// Bounds checks the two vertices and then returns the
    /// canonicalized edge.
    fn edge(&self, u: usize, v: usize) -> Edge {
        self.bounds_check(u);
        self.bounds_check(v);
        Edge::new(u, v)
    }

    /// Bounds checks a vertex, panicking if out of bounds.
    fn bounds_check(&self, v: usize) {
        assert!(
            v < self.len(),
            "Vertex {} not in bounds for graph of size {}",
            v,
            self.len()
        );
    }
}

impl<'a> IndexGraph<'a> for EdgeSetGraph {
    type Neighbors = Neighbors<'a>;

    fn len(&self) -> usize {
        self.len
    }

    fn new_vertex(&mut self) -> usize {
        let result = self.len;
        self.len = result + 1;
        result
    }

    fn set_edge(&mut self, u: usize, v: usize, present: bool) {
        let edge = self.edge(u, v);
        if present {
            self.edges.insert(edge);
        } else {
            self.edges.remove(&edge);
        }
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        let edge = self.edge(u, v);
        self.edges.contains(&edge)
    }

    fn neighbors(&'a self, u: usize) -> Self::Neighbors {
        Neighbors {
            graph: self,
            vertex: u,
            next: 0,
        }
    }
}

impl std::default::Default for EdgeSetGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Neighbors<'a> {
    graph: &'a EdgeSetGraph,
    vertex: Vertex,
    next: Vertex,
}

impl Iterator for Neighbors<'_> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        while self.next < self.graph.len() {
            let candidate = self.next;
            self.next += 1;

            if self.graph.has_edge(self.vertex, candidate) {
                return Some(candidate);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.graph.len() - self.next))
    }
}

#[cfg(test)]
mod tests {
    use super::{EdgeSetGraph as Graph, IndexGraph, Vertex};

    impl Graph {
        /// Helper function for asserting that the neighbors of `u` are
        /// `expected`.
        fn check_neighbors(&self, u: Vertex, expected: &[Vertex]) {
            use std::iter::FromIterator;

            let actual = Vec::from_iter(self.neighbors(u));
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn neighbors() {
        let g = Graph::from_edges(vec![(5, 0), (0, 1), (0, 2), (1, 2), (2, 2), (1, 3), (2, 3)]);

        g.check_neighbors(0, &[1, 2, 5]);
        g.check_neighbors(1, &[0, 2, 3]);
        g.check_neighbors(2, &[0, 1, 2, 3]);
        g.check_neighbors(3, &[1, 2]);
        g.check_neighbors(4, &[]);
        g.check_neighbors(5, &[0]);
    }

    #[test]
    fn new_vertex_and_len() {
        let mut g = Graph::new();
        assert_eq!(g.len(), 0);
        assert_eq!(g.new_vertex(), 0);
        assert_eq!(g.new_vertex(), 1);
        assert_eq!(g.len(), 2);
    }

    #[test]
    fn add_remove_has() {
        let mut g = Graph::new();
        assert_eq!(g.len(), 0);
        for i in 0..5 {
            assert_eq!(g.new_vertex(), i);
        }

        assert!(!g.has_edge(0, 1));
        g.add_edge(0, 1);
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 0));
        assert!(!g.has_edge(0, 2));
    }

    #[test]
    fn from_edges() {
        let edges = &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0), (5, 2)];
        let g = Graph::from_edges(edges.iter().copied());

        for &(u, v) in edges {
            assert!(g.has_edge(u, v));
            assert!(g.has_edge(v, u));
        }

        assert!(!g.has_edge(0, 2));
        assert!(!g.has_edge(0, 3));
        assert!(!g.has_edge(1, 5));
    }
}
