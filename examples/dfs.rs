use algo_graph::{dfs::dfs, graph::Graph};

fn main() {
    let g = Graph::from_adjacency_list([vec![(1, 1), (2, 1)], vec![(3, 1)], vec![(3, 1)], vec![]]);

    println!("{:?}", dfs(&g));
}
