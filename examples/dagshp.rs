use algo_graph::{dagshp::dagshp, graph::Graph};
use std::fs::File;

fn main() {
    let g = Graph::from_adjacency_list([
        vec![(1, 1), (3, 3), (4, 3)],
        vec![(4, 1), (5, 3)],
        vec![(1, 1), (5, 3)],
        vec![],
        vec![(3, 0), (5, 2)],
        vec![],
    ]);

    match dagshp(&g, 1) {
        Ok(dagshp_output) => {
            println!("{dagshp_output:?}");
            let mut f = File::create("dagshp.dot").expect("could not create file");
            dot::render(&dagshp_output, &mut f).expect("could not save output to file");
        }
        Err((v, _)) => println!("input graph is not a dag. {v} is a vertex in the cycle found"),
    }
}
