use crate::{
    dfs::Color,
    graph::{Graph, Vertex},
};

use infinitable::Infinitable::{self, *};

#[derive(Debug, Clone)]
pub struct DAGshPOutput<const N: usize> {
    pub d: [Infinitable<isize>; N],
    pub pi: [Option<Vertex>; N],
}

type DAGshPResult<const N: usize> = Result<DAGshPOutput<N>, (Vertex, [Option<Vertex>; N])>;

pub fn dagshp<const N: usize>(graph: &Graph<N>, s: Vertex) -> DAGshPResult<N> {
    let stack = topological_sort(graph, s)?;
    Ok(dagshps(graph, stack))
}

fn topological_sort<const N: usize>(
    graph: &Graph<N>,
    s: Vertex,
) -> Result<Vec<Vertex>, (Vertex, [Option<Vertex>; N])> {
    let mut color = [Color::White; N];
    let mut pi = [None; N];
    let mut stack = Vec::new();

    match df_visit(graph, s, &mut color, &mut pi, &mut stack) {
        Ok(()) => Ok(stack),
        Err(deg) => Err((deg, pi)),
    }
}

fn df_visit<const N: usize>(
    graph: &Graph<N>,
    u: Vertex,
    color: &mut [Color; N],
    pi: &mut [Option<Vertex>; N],
    stack: &mut Vec<Vertex>,
) -> Result<(), Vertex> {
    color[u] = Color::Grey;

    for &(v, _) in &graph.adjacency_list[u] {
        if color[v] == Color::White {
            pi[v] = Some(u);
            df_visit(graph, v, color, pi, stack)?;
        } else if color[v] == Color::Grey {
            pi[v] = Some(u);
            return Err(v);
        }
    }

    color[u] = Color::Black;
    stack.push(u);

    Ok(())
}

fn dagshps<const N: usize>(graph: &Graph<N>, mut stack: Vec<Vertex>) -> DAGshPOutput<N> {
    let mut d = [Infinity; N];
    let mut pi = [None; N];

    let s = *stack
        .last()
        .expect("at least one element should be in stack");
    d[s] = Finite(0);

    while let Some(u) = stack.pop() {
        for &(v, w) in &graph.adjacency_list[u] {
            let ed = Finite(w + d[u].finite().expect("d[u] should be finite by this point"));
            if d[v] > ed {
                pi[v] = Some(u);
                d[v] = ed;
            }
        }
    }

    DAGshPOutput { d, pi }
}
