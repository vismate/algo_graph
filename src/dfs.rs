use crate::graph::{Graph, Vertex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Grey,
    Black,
}

#[derive(Debug, Clone)]
pub struct DFSOutput<const N: usize> {
    d: [usize; N],
    f: [usize; N],
    pi: [Option<Vertex>; N],
    topological_stack: Option<Vec<Vertex>>,
}

pub fn dfs<const N: usize>(graph: &Graph<N>) -> DFSOutput<N> {
    let mut time: usize = 0;
    let mut color = [Color::White; N];
    let mut d = [0; N];
    let mut f = [0; N];
    let mut pi = [None; N];
    let mut topological_stack = Some(Vec::new());

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
            );
        }
    }

    DFSOutput {
        d,
        f,
        pi,
        topological_stack,
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs_visit<const N: usize>(
    graph: &Graph<N>,
    u: Vertex,
    time: &mut usize,
    color: &mut [Color; N],
    d: &mut [usize; N],
    f: &mut [usize; N],
    pi: &mut [Option<Vertex>; N],
    topological_stack: &mut Option<Vec<Vertex>>,
) {
    color[u] = Color::Grey;
    *time += 1;
    d[u] = *time;
    for &(v, _) in &graph.adjacency_list[u] {
        if color[v] == Color::White {
            pi[v] = Some(u);
            dfs_visit(graph, v, time, color, d, f, pi, topological_stack);
        } else if color[v] == Color::Grey {
            topological_stack.take();
        }
    }

    *time += 1;
    f[u] = *time;
    color[u] = Color::Black;
    if let Some(s) = topological_stack {
        s.push(u);
    }
}
