use crate::graph::{Graph, Vertex};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BFSOutput<const N: usize> {
    d: [usize; N],
    pi: [Option<Vertex>; N],
}

pub fn bfs<const N: usize>(graph: &Graph<N>, s: Vertex) -> BFSOutput<N> {
    let mut d = [usize::MAX; N];
    let mut pi = [None; N];

    let mut q = VecDeque::with_capacity(N);
    d[s] = 0;

    q.push_back(s);

    while let Some(u) = q.pop_front() {
        for &(v, _) in &graph.adjacency_list[u] {
            if d[v] == usize::MAX {
                d[v] = d[u] + 1;
                pi[v] = Some(u);
                q.push_back(v);
            }
        }
    }

    BFSOutput { d, pi }
}
