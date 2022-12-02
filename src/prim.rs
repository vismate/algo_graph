use crate::graph::{Graph, Vertex};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct PrimOutput<const N: usize> {
    c: [isize; N],
    p: [Option<Vertex>; N],
}

pub fn prim<const N: usize>(graph: &Graph<N>, r: Vertex) -> PrimOutput<N> {
    let mut c = [isize::MAX; N];
    let mut p = [None; N];

    c[r] = 0;
    let mut q: PriorityQueue<Vertex, Reverse<isize>> =
        (0..N).zip(c.iter().copied().map(Reverse)).collect();

    while let Some((u, _)) = q.pop() {
        for &(v, w) in &graph.adjacency_list[u] {
            if q.get(&v).is_some() && c[v] > w {
                p[v] = Some(u);
                c[v] = w;
                q.change_priority(&v, Reverse(c[v]));
            }
        }
    }

    PrimOutput { c, p }
}
