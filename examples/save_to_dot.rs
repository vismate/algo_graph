use algo_graph::graph::Graph;
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

    let mut f = File::create("graph.dot").expect("could not create file");
    dot::render(&g, &mut f).expect("could not render dot file");
}
