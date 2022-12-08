use crate::graph::{Graph, Vertex};
use infinitable::Infinitable::{self, *};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct PrimOutput<const N: usize> {
    pub c: [Infinitable<isize>; N],
    pub p: [Option<Vertex>; N],
}

pub fn prim<const N: usize>(graph: &Graph<N>, r: Vertex) -> PrimOutput<N> {
    let mut c = [Infinity; N];
    let mut p = [None; N];

    c[r] = Finite(0);
    let mut q: PriorityQueue<Vertex, Reverse<Infinitable<isize>>> =
        (0..N).zip(c.iter().copied().map(Reverse)).collect();

    while let Some((u, _)) = q.pop() {
        for &(v, w) in &graph.adjacency_list[u] {
            let w = Finite(w);
            if q.get(&v).is_some() && c[v] > w {
                p[v] = Some(u);
                c[v] = w;
                q.change_priority(&v, Reverse(c[v]));
            }
        }
    }

    PrimOutput { c, p }
}
