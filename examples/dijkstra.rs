use algo_graph::{dijkstra::dijkstra, graph::Graph};
use std::fs::File;

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 1), (3, 3), (4, 3)],
        vec![(0, 1), (4, 1), (5, 3)],
        vec![],
        vec![(0, 3), (4, 0)],
        vec![(0, 3), (1, 1), (3, 0), (5, 2)],
        vec![(1, 3), (4, 2)],
    ]);

    let dijkstra_output = dijkstra(&g, 0);
    println!("{dijkstra_output:?}");

    let mut f = File::create("dijkstra.dot").expect("could not create file");
    dot::render(&dijkstra_output, &mut f).expect("could not save putput to file");
}
