use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

pub type Vertex = usize;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Edge {
    pub u: Vertex,
    pub v: Vertex,
    pub w: i32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.w.cmp(&other.w)
    }
}

#[derive(Debug, Clone)]
pub struct Graph<const N: usize> {
    pub adjacency_list: [Vec<(Vertex, i32)>; N],
}

impl<const N: usize> Graph<N> {
    pub fn from_adjacency_list(adjacency_list: [Vec<(Vertex, i32)>; N]) -> Self {
        Self { adjacency_list }
    }

    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.adjacency_list
            .iter()
            .enumerate()
            .flat_map(|(u, l)| l.iter().map(move |&(v, w)| Edge { u, v, w }))
    }
}
