use crate::graph::{Edge, Graph, Vertex};
use dot::{GraphWalk, Id, LabelText, Labeller};

impl<'a, const N: usize> Labeller<'a, Vertex, Edge> for Graph<N> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("my_graph").unwrap()
    }

    fn node_id(&'a self, n: &Vertex) -> Id<'a> {
        Id::new(format!("N{n}")).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Vertex) -> LabelText<'b> {
        LabelText::LabelStr(String::from(('a'..='z').nth(*n).unwrap()).into())
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
