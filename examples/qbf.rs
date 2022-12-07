use algo_graph::{graph::Graph, qbf::qbf};

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 3), (2, 1)],
        vec![(3, -2)],
        vec![(1, -1)],
        vec![(2, 3)],
    ]);

    println!("{:?}", qbf(&g, 0));
}
