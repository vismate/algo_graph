#![feature(let_chains)]
mod bfs;
mod dfs;
mod graph;
mod kruskal;
mod qbf;

use bfs::bfs;
use dfs::dfs;
use graph::Graph;
use kruskal::kruskal;
use qbf::qbf;

//TODO: Prim, Dijkstra, DAGshP

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
        vec![(1, 3), (2, 1)],
        vec![(3, -2)],
        vec![(1, -1)],
        vec![(2, 3)],
    ]);

    println!("{:?}", qbf(&g4, 0));
}
