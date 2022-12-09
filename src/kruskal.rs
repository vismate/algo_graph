use crate::graph::{Edge, Graph, Vertex};
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct KruskalOutput(Vec<Edge>);

pub type KruskalResult = Result<KruskalOutput, usize>;

pub fn kruskal<const N: usize>(graph: &Graph<N>) -> KruskalResult {
    use std::collections::BinaryHeap;

    let mut pi = [0; N];
    let mut s = [1; N];
    let mut k = N;
    let mut a = Vec::new();
    let mut q: BinaryHeap<_> = graph.edges().map(Reverse).collect();

    for v in 0..N {
        make_set(v, &mut pi, &mut s);
    }

    while k > 1 && let Some(Reverse(e)) = q.pop(){
        let x = find_set(e.u, &mut pi); let y = find_set(e.v, &mut pi);
        if x != y {
            k -= 1;
            a.push(e);
            union_set(x,y, &mut pi, &mut s);
        }
    }

    if k == 1 {
        Ok(KruskalOutput(a))
    } else {
        Err(k)
    }
}

fn make_set<const N: usize>(v: Vertex, pi: &mut [Vertex; N], s: &mut [usize; N]) {
    pi[v] = v;
    s[v] = 1;
}

fn find_set<const N: usize>(v: Vertex, pi: &mut [Vertex; N]) -> Vertex {
    if pi[v] != v {
        pi[v] = find_set(pi[v], pi);
    }
    pi[v]
}

fn union_set<const N: usize>(x: Vertex, y: Vertex, pi: &mut [Vertex; N], s: &mut [usize; N]) {
    if s[x] < s[y] {
        pi[x] = y;
        s[y] += s[x];
    } else {
        pi[y] = x;
        s[x] += s[y];
    }
}
