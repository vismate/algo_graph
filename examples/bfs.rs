use algo_graph::{bfs::bfs, graph::Graph};
use std::fs::File;

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 1)],
        vec![(2, 1), (4, 1)],
        vec![(4, 1)],
        vec![(0, 1)],
        vec![(3, 1)],
        vec![(2, 1), (4, 1)],
    ]);

    let bfs_output = bfs(&g, 0);
    println!("{bfs_output:?}");

    let mut f = File::create("bfs_output.dot").expect("could not create file");
    dot::render(&bfs_output, &mut f).expect("could not save output to file");
}
