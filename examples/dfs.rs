use algo_graph::{dfs::dfs, graph};
use std::fs::File;

fn main() {
    let g = graph! {
        Nodes: 4;

        a -> b ; c.
        b -> d.
        c -> d.
        d -> .
    }
    .expect("could not parse graph");

    let dfs_output = dfs(&g);
    println!("{dfs_output:?}");

    let mut f = File::create("dfs_output.dot").expect("could not create file");
    dot::render(&dfs_output, &mut f).expect("could not save output to file");
}
