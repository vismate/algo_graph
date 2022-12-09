use algo_graph::{graph::Graph, kruskal::kruskal};
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

    match kruskal(&g) {
        Ok(kruskal_output) => {
            println!("{kruskal_output:?}");
            let mut f = File::create("kruskal.dot").expect("could not create file");
            dot::render(&kruskal_output, &mut f).expect("could not save file to output");
        }
        Err(k) => println!("more than one ({k}) component in supplied graph"),
    }
}
