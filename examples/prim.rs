use algo_graph::{graph::Graph, prim::prim};

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 0), (3, 2), (4, 2)],
        vec![(0, 0), (2, 1), (4, 1), (5, 2)],
        vec![(1, 1), (5, 3)],
        vec![(0, 2), (4, 0)],
        vec![(0, 2), (1, 1), (3, 0), (5, 2)],
        vec![(1, 2), (2, 3), (4, 2)],
    ]);

    println!("{:?}", prim(&g, 3));
}
