use crate::graph::{Graph, Vertex};
use infinitable::Infinitable::{self, *};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BFSOutput<const N: usize> {
    d: [Infinitable<usize>; N],
    pi: [Option<Vertex>; N],
}

pub fn bfs<const N: usize>(graph: &Graph<N>, s: Vertex) -> BFSOutput<N> {
    let mut d = [Infinity; N];
    let mut pi = [None; N];

    let mut q = VecDeque::with_capacity(N);
    d[s] = Finite(0);

    q.push_back(s);

    while let Some(u) = q.pop_front() {
        for &(v, _) in &graph.adjacency_list[u] {
            if d[v] == Infinity {
                d[v] = Finite(d[u].finite().expect("d[u] should be finite by this point") + 1);
                pi[v] = Some(u);
                q.push_back(v);
            }
        }
    }

    BFSOutput { d, pi }
}
