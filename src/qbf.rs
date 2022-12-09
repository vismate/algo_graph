use crate::graph::{Graph, Vertex};
use infinitable::Infinitable::{self, *};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct QBFOutput<const N: usize> {
    pub d: [Infinitable<isize>; N],
    pub pi: [Option<Vertex>; N],
}

pub type QBFResult<const N: usize> = Result<QBFOutput<N>, (Vertex, [Option<Vertex>; N])>;

pub fn qbf<const N: usize>(graph: &Graph<N>, s: Vertex) -> QBFResult<N> {
    let mut d = [Infinity; N];
    let mut pi = [None; N];
    let mut e = [0; N];
    let mut inq = [false; N];
    let mut q = VecDeque::with_capacity(N);

    d[s] = Finite(0);
    q.push_back(s);
    inq[s] = true;

    while let Some(u) = q.pop_front() {
        inq[u] = false;

        for &(v, w) in &graph.adjacency_list[u] {
            let ed = Finite(w + d[u].finite().expect("d[u] should be finite by this point"));
            if d[v] > ed {
                d[v] = ed;
                pi[v] = Some(u);
                e[v] += 1;

                if e[v] < N {
                    if !inq[v] {
                        q.push_back(v);
                        inq[v] = true;
                    }
                } else {
                    return Err((find_neg_cycle(v, &pi), pi));
                }
            }
        }
    }

    Ok(QBFOutput { d, pi })
}

fn find_neg_cycle<const N: usize>(mut v: Vertex, pi: &[Option<Vertex>; N]) -> Vertex {
    let mut b = [false; N];

    loop {
        b[v] = true;
        v = pi[v].expect("in case of a negative cycle v should always have a parent");

        if b[v] {
            break v;
        }
    }
}
