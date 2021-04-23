pub trait IndexGraph<'a> {
    type Neighbors: Iterator<Item = usize>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn new_vertex(&mut self) -> usize;

    fn set_edge(&mut self, u: usize, v: usize, present: bool);

    fn add_edge(&mut self, u: usize, v: usize) {
        self.set_edge(u, v, true);
    }

    fn remove_edge(&mut self, u: usize, v: usize) {
        self.set_edge(u, v, false);
    }

    fn has_edge(&self, u: usize, v: usize) -> bool;

    fn neighbors(&'a self, v: usize) -> Self::Neighbors;
}
