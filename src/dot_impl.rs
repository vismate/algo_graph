use crate::{
    bfs::BFSOutput,
    graph::{Edge, Graph, Vertex},
};
use dot::{GraphWalk, Id, LabelText, Labeller};

impl<'a, const N: usize> Labeller<'a, Vertex, Edge> for Graph<N> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("my_graph").unwrap()
    }

    fn node_id(&'a self, n: &Vertex) -> Id<'a> {
        Id::new(format!("N{n}")).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Vertex) -> LabelText<'b> {
        LabelText::LabelStr(
            String::from(
                ('a'..='z')
                    .nth(*n)
                    .expect("cannot label more than 26 nodes at the moment"),
            )
            .into(),
        )
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
        Id::new(format!("N{n}")).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Vertex) -> LabelText<'b> {
        LabelText::LabelStr(
            format!(
                "{}:{}",
                ('a'..='z')
                    .nth(*n)
                    .expect("cannot label more than 26 nodes at the moment"),
                self.d[*n].finite().map_or("inf".into(), |v| v.to_string())
            )
            .into(),
        )
    }
}

impl<'a, const N: usize> GraphWalk<'a, Vertex, Edge> for BFSOutput<N> {
    fn nodes(&self) -> dot::Nodes<'a, Vertex> {
        (0..N).collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge> {
        self.pi
            .iter()
            .enumerate()
            .filter_map(|(v, u)| u.map(|u| Edge { u, v, w: 0 }))
            .collect()
    }

    fn source(&'a self, e: &Edge) -> Vertex {
        e.u
    }

    fn target(&self, e: &Edge) -> Vertex {
        e.v
    }
}
