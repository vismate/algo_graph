use algo_graph::{dfs::dfs, graph::Graph};
use std::fs::File;

fn main() {
    let g = Graph::from_adjacency_list([vec![(1, 1), (2, 1)], vec![(3, 1)], vec![(3, 1)], vec![]]);

    let dfs_output = dfs(&g);
    println!("{dfs_output:?}");

    let mut f = File::create("dfs_output.dot").expect("could not create file");
    dot::render(&dfs_output, &mut f).expect("could not save output to file");
}
