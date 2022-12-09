use algo_graph::{graph::Graph, qbf::qbf};
use std::fs::File;

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 3), (2, 1)],
        vec![(3, -2)],
        vec![(1, -1)],
        vec![(2, 3)],
    ]);

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
