use algo_graph::{graph::Graph, prim::prim};
use std::fs::File;

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 0), (3, 2), (4, 2)],
        vec![(0, 0), (2, 1), (4, 1), (5, 2)],
        vec![(1, 1), (5, 3)],
        vec![(0, 2), (4, 0)],
        vec![(0, 2), (1, 1), (3, 0), (5, 2)],
        vec![(1, 2), (2, 3), (4, 2)],
    ]);

    let prim_output = prim(&g, 3);

    println!("{prim_output:?}");

    let mut f = File::create("prim.dot").expect("could not create file");
    dot::render(&prim_output, &mut f).expect("could not save output to file");
}
