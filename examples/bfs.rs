use algo_graph::{bfs::bfs, graph};
use std::fs::File;

fn main() {
    let g = graph! {
        Nodes: 6;

        a -> b.
        b -> c ; e.
        c -> e.
        d -> a.
        e -> d.
        f -> c ; e.
    }
    .expect("could not parse graph");

    let bfs_output = bfs(&g, 0);
    println!("{bfs_output:?}");

    let mut f = File::create("bfs_output.dot").expect("could not create file");
    dot::render(&bfs_output, &mut f).expect("could not save output to file");
}
