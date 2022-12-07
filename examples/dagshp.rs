use algo_graph::{dagshp::dagshp, graph::Graph};

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 1), (3, 3), (4, 3)],
        vec![(4, 1), (5, 3)],
        vec![(1, 1), (5, 3)],
        vec![],
        vec![(3, 0), (5, 2)],
        vec![],
    ]);

    println!("{:?}", dagshp(&g, 0));
}
