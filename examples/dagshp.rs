use algo_graph::{dagshp::dagshp, graph};
use std::fs::File;

fn main() {
    let g = graph! {
        Nodes: 6;

        a -> b, 1 ; d, 3 ; e, 3.
        b -> e, 1 ; f, 3.
        c -> b, 1 ; f, 3.
        d -> .
        e -> d, 0 ; f, 2.
        f -> .

    }
    .expect("could not parse graph");

    match dagshp(&g, 0) {
        Ok(dagshp_output) => {
            println!("{dagshp_output:?}");
            let mut f = File::create("dagshp.dot").expect("could not create file");
            dot::render(&dagshp_output, &mut f).expect("could not save output to file");
        }
        Err((v, _)) => println!("input graph is not a dag. {v} is a vertex in the cycle found"),
    }
}
