use crate::graph::{Graph, Vertex};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct DijkstraOutput<const N: usize> {
    d: [isize; N],
    pi: [Option<Vertex>; N],
}

pub fn dijkstra<const N: usize>(graph: &Graph<N>, s: Vertex) -> DijkstraOutput<N> {
    let mut d = [isize::MAX; N];
    let mut pi = [None; N];

    d[s] = 0;
    let mut q: PriorityQueue<Vertex, Reverse<isize>> =
        (0..N).zip(d.iter().copied().map(Reverse)).collect();

    while let Some((u, _)) = q.pop() && d[u] < isize::MAX {
        for &(v, w) in &graph.adjacency_list[u] {
            if d[v] > d[u] + w {
                pi[v] = Some(u);
                d[v] = d[u] + w;
                q.change_priority(&v, Reverse(d[v]));
            }
        }
    }

    DijkstraOutput { d, pi }
}
