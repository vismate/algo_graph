use algo_graph::{bfs::bfs, graph::Graph};

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 1)],
        vec![(2, 1), (4, 1)],
        vec![(4, 1)],
        vec![(0, 1)],
        vec![(3, 1)],
        vec![(2, 1), (4, 1)],
    ]);

    println!("{:?}", bfs(&g, 0));
}
