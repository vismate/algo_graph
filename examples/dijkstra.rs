use algo_graph::{dijkstra::dijkstra, graph::Graph};

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 1), (3, 3), (4, 3)],
        vec![(0, 1), (4, 1), (5, 3)],
        vec![],
        vec![(0, 3), (4, 0)],
        vec![(0, 3), (1, 1), (3, 0), (5, 2)],
        vec![(1, 3), (4, 2)],
    ]);

    println!("{:?}", dijkstra(&g, 0));
}
