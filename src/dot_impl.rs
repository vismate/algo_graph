use crate::{
    bfs::BFSOutput,
    dfs::{DFSOutput, EdgeKind},
    graph::{Edge, Graph, Vertex},
};
use dot::{GraphWalk, Id, LabelText, Labeller};
use infinitable::Infinitable;

fn infinitable_to_string<T: std::fmt::Display>(v: &Infinitable<T>) -> String {
    match v {
        Infinitable::Finite(n) => n.to_string(),
        Infinitable::Infinity => "inf".into(),
        Infinitable::NegativeInfinity => "-inf".into(),
    }
}

fn node_id<'a>(n: Vertex) -> Id<'a> {
    Id::new(format!("N{n}")).unwrap()
}

fn node_label_char(n: Vertex) -> char {
    ('a'..='z')
        .nth(n)
        .expect("cannot label more than 26 nodes at the moment")
}

fn node_label_with_value<'a, T: std::fmt::Display>(
    n: Vertex,
    value: &Infinitable<T>,
) -> LabelText<'a> {
    LabelText::LabelStr(format!("{} : {}", node_label_char(n), infinitable_to_string(value)).into())
}

fn edges_from_pi<'a, const N: usize>(pi: &[Option<Vertex>; N]) -> dot::Edges<'a, Edge> {
    pi.iter()
        .enumerate()
        .filter_map(|(v, u)| u.map(|u| Edge { u, v, w: 0 }))
        .collect()
}

impl<'a, const N: usize> Labeller<'a, Vertex, Edge> for Graph<N> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("my_graph").unwrap()
    }

    fn node_id(&'a self, n: &Vertex) -> Id<'a> {
        node_id(*n)
    }

    fn node_label<'b>(&'b self, n: &Vertex) -> LabelText<'b> {
        LabelText::LabelStr(node_label_char(*n).to_string().into())
    }

    fn edge_label<'b>(&'b self, e: &Edge) -> LabelText<'b> {
        LabelText::LabelStr(e.w.to_string().into())
    }
}

impl<'a, const N: usize> GraphWalk<'a, Vertex, Edge> for Graph<N> {
    fn nodes(&self) -> dot::Nodes<'a, Vertex> {
        (0..N).collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, Edge> {
        self.edges().collect()
    }
    fn source(&self, e: &Edge) -> Vertex {
        e.u
    }
    fn target(&self, e: &Edge) -> Vertex {
        e.v
    }
}

impl<'a, const N: usize> Labeller<'a, Vertex, Edge> for BFSOutput<N> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("bfs_output_graph").unwrap()
    }

    fn node_id(&'a self, n: &Vertex) -> Id<'a> {
        node_id(*n)
    }

    fn node_label<'b>(&'b self, n: &Vertex) -> LabelText<'b> {
        node_label_with_value(*n, &self.d[*n])
    }
}

impl<'a, const N: usize> GraphWalk<'a, Vertex, Edge> for BFSOutput<N> {
    fn nodes(&self) -> dot::Nodes<'a, Vertex> {
        (0..N).collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge> {
        edges_from_pi(&self.pi)
    }

    fn source(&'a self, e: &Edge) -> Vertex {
        e.u
    }

    fn target(&self, e: &Edge) -> Vertex {
        e.v
    }
}

impl<'a, const N: usize> Labeller<'a, Vertex, (Edge, EdgeKind)> for DFSOutput<N> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("dfs_output_graph").unwrap()
    }

    fn node_id(&'a self, n: &Vertex) -> Id<'a> {
        node_id(*n)
    }

    fn node_label<'b>(&'b self, n: &Vertex) -> LabelText<'b> {
        LabelText::LabelStr(
            format!(
                "{} : {}/{}",
                node_label_char(*n),
                infinitable_to_string(&self.d[*n]),
                infinitable_to_string(&self.f[*n]),
            )
            .into(),
        )
    }
    fn edge_label<'b>(&'b self, e: &(Edge, EdgeKind)) -> LabelText<'b> {
        LabelText::LabelStr(format!("{:?} : {}", e.1, e.0.w).into())
    }
}

impl<'a, const N: usize> GraphWalk<'a, Vertex, (Edge, EdgeKind)> for DFSOutput<N> {
    fn nodes(&self) -> dot::Nodes<'a, Vertex> {
        (0..N).collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, (Edge, EdgeKind)> {
        self.classified_edges.as_slice().into()
    }

    fn source(&'a self, e: &(Edge, EdgeKind)) -> Vertex {
        e.0.u
    }

    fn target(&'a self, e: &(Edge, EdgeKind)) -> Vertex {
        e.0.v
    }
}
