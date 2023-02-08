use algo_graph::{dijkstra::dijkstra, graph};
use std::fs::File;

fn main() {
    let g = graph! {
        Nodes: 6;

        a - b, 1 ; d, 3 ; e, 3.
        b - e, 1 ; f, 3.
        c - .
        d - e, 0.
        e - f, 2.
    }
    .expect("could not parse graph");

    let dijkstra_output = dijkstra(&g, 0);
    println!("{dijkstra_output:?}");

    let mut f = File::create("dijkstra.dot").expect("could not create file");
    dot::render(&dijkstra_output, &mut f).expect("could not save putput to file");
}
