use algo_graph::{graph, qbf::qbf};
use std::fs::File;

fn main() {
    let g = graph! {
        Nodes: 4;

        a -> b, 3 ; c, 1.
        b -> d, -2.
        c -> b, -1.
        d -> c, 3.
    }
    .expect("could not parse graph");

    match qbf(&g, 0) {
        Ok(qbf_output) => {
            println!("{qbf_output:?}");
            let mut f = File::create("qbf.dot").expect("could not open file");
            dot::render(&qbf_output, &mut f).expect("could not save output to file");
        }
        Err((v, _)) => {
            println!("negative cycle found in input graph. {v} is a vertex in the cycle");
        }
    }
}
