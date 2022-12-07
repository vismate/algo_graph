use crate::graph::{Graph, Vertex};
use infinitable::Infinitable::{self, *};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct DijkstraOutput<const N: usize> {
    d: [Infinitable<isize>; N],
    pi: [Option<Vertex>; N],
}

pub fn dijkstra<const N: usize>(graph: &Graph<N>, s: Vertex) -> DijkstraOutput<N> {
    let mut d = [Infinity; N];
    let mut pi = [None; N];

    d[s] = Finite(0);
    let mut q: PriorityQueue<Vertex, Reverse<Infinitable<isize>>> =
        (0..N).zip(d.iter().copied().map(Reverse)).collect();

    while let Some((u, _)) = q.pop() && d[u] < Infinity {
        for &(v, w) in &graph.adjacency_list[u] {
            let ed = Finite(w + d[u].finite().expect("d[u] should be finite by this point"));
            if d[v] > ed {
                pi[v] = Some(u);
                d[v] = ed;
                q.change_priority(&v, Reverse(d[v]));
            }
        }
    }

    DijkstraOutput { d, pi }
}
