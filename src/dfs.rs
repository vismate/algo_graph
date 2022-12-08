use crate::graph::{Edge, Graph, Vertex};
use infinitable::Infinitable::{self, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Grey,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum EdgeKind {
    Tree,
    Forward,
    Backward,
    Cross,
}

#[derive(Debug, Clone)]
pub struct DFSOutput<const N: usize> {
    pub d: [Infinitable<usize>; N],
    pub f: [Infinitable<usize>; N],
    pub pi: [Option<Vertex>; N],
    pub topological_stack: Option<Vec<Vertex>>,
    pub classified_edges: Vec<(Edge, EdgeKind)>,
}

pub fn dfs<const N: usize>(graph: &Graph<N>) -> DFSOutput<N> {
    let mut time: usize = 0;
    let mut color = [Color::White; N];
    let mut d = [Infinity; N];
    let mut f = [Infinity; N];
    let mut pi = [None; N];
    let mut topological_stack = Some(Vec::new());
    let mut classified_edges = Vec::new();

    for u in 0..N {
        if color[u] == Color::White {
            dfs_visit(
                graph,
                u,
                &mut time,
                &mut color,
                &mut d,
                &mut f,
                &mut pi,
                &mut topological_stack,
                &mut classified_edges,
            );
        }
    }

    DFSOutput {
        d,
        f,
        pi,
        topological_stack,
        classified_edges,
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs_visit<const N: usize>(
    graph: &Graph<N>,
    u: Vertex,
    time: &mut usize,
    color: &mut [Color; N],
    d: &mut [Infinitable<usize>; N],
    f: &mut [Infinitable<usize>; N],
    pi: &mut [Option<Vertex>; N],
    topological_stack: &mut Option<Vec<Vertex>>,
    classified_edges: &mut Vec<(Edge, EdgeKind)>,
) {
    color[u] = Color::Grey;
    *time += 1;
    d[u] = Finite(*time);
    for &(v, w) in &graph.adjacency_list[u] {
        match color[v] {
            Color::White => {
                pi[v] = Some(u);
                classified_edges.push((Edge { u, v, w }, EdgeKind::Tree));
                dfs_visit(
                    graph,
                    v,
                    time,
                    color,
                    d,
                    f,
                    pi,
                    topological_stack,
                    classified_edges,
                );
            }
            Color::Grey => {
                topological_stack.take();
                classified_edges.push((Edge { u, v, w }, EdgeKind::Backward));
            }

            Color::Black => {
                let kind = if d[u] < d[v] {
                    EdgeKind::Forward
                } else {
                    EdgeKind::Cross
                };
                classified_edges.push((Edge { u, v, w }, kind));
            }
        }
    }

    *time += 1;
    f[u] = Finite(*time);
    color[u] = Color::Black;
    if let Some(s) = topological_stack {
        s.push(u);
    }
}
