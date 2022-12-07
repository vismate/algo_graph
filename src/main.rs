use algo_graph::*;
use bfs::bfs;
use dagshp::dagshp;
use dfs::dfs;
use dijkstra::dijkstra;
use graph::Graph;
use kruskal::kruskal;
use prim::prim;
use qbf::qbf;

fn main() {
    let g1 = Graph::from_adjacency_list([
        vec![(1, 1)],
        vec![(2, 1), (4, 1)],
        vec![(4, 1)],
        vec![(0, 1)],
        vec![(3, 1)],
        vec![(2, 1), (4, 1)],
    ]);

    println!("{:?}", bfs(&g1, 0));

    let g2 = Graph::from_adjacency_list([vec![(1, 1), (2, 1)], vec![(3, 1)], vec![(3, 1)], vec![]]);

    println!("{:?}", dfs(&g2));

    let g3 = Graph::from_adjacency_list([
        vec![(1, 0), (3, 2), (4, 2)],
        vec![(0, 0), (2, 1), (4, 1), (5, 2)],
        vec![(1, 1), (5, 3)],
        vec![(0, 2), (4, 0)],
        vec![(0, 2), (1, 1), (3, 0), (5, 2)],
        vec![(1, 2), (2, 3), (4, 2)],
    ]);

    println!("{:?}", kruskal(&g3));

    let g4 = Graph::from_adjacency_list([
        vec![(1, 0), (3, 2), (4, 2)],
        vec![(0, 0), (2, 1), (4, 1), (5, 2)],
        vec![(1, 1), (5, 3)],
        vec![(0, 2), (4, 0)],
        vec![(0, 2), (1, 1), (3, 0), (5, 2)],
        vec![(1, 2), (2, 3), (4, 2)],
    ]);

    println!("{:?}", prim(&g4, 3));

    let g5 = Graph::from_adjacency_list([
        vec![(1, 1), (3, 3), (4, 3)],
        vec![(0, 1), (4, 1), (5, 3)],
        vec![],
        vec![(0, 3), (4, 0)],
        vec![(0, 3), (1, 1), (3, 0), (5, 2)],
        vec![(1, 3), (4, 2)],
    ]);

    println!("{:?}", dijkstra(&g5, 0));

    let g6 = Graph::from_adjacency_list([
        vec![(1, 1), (3, 3), (4, 3)],
        vec![(4, 1), (5, 3)],
        vec![(1, 1), (5, 3)],
        vec![],
        vec![(3, 0), (5, 2)],
        vec![],
    ]);

    println!("{:?}", dagshp(&g6, 0));

    let g7 = Graph::from_adjacency_list([
        vec![(1, 3), (2, 1)],
        vec![(3, -2)],
        vec![(1, -1)],
        vec![(2, 3)],
    ]);

    println!("{:?}", qbf(&g7, 0));

    use std::fs::File;
    let mut f = File::create("graph.dot").unwrap();
    dot::render(&g7, &mut f).unwrap();
}
