use algo_graph::{graph, prim::prim};
use std::fs::File;

fn main() {
    let g = graph! {
        Nodes: 6;

        a - b, 0 ; d, 2 ; e, 2.
        b - c, 1 ; e, 1 ; f, 2.
        c - f, 3.
        d - e, 0.
        e - f, 2.
        f - .
    }
    .expect("could not parse graph");

    let prim_output = prim(&g, 3);

    println!("{prim_output:?}");

    let mut f = File::create("prim.dot").expect("could not create file");
    dot::render(&prim_output, &mut f).expect("could not save output to file");
}
