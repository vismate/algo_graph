use algo_graph::{graph, kruskal::kruskal};
use std::fs::File;

fn main() {
    let g = graph! {
        Nodes: 6;

        a - b, 0 ; d, 2 ; e, 2.
        b - c, 1 ; e, 1 ; f, 2.
        c - f, 3.
        d - e, 0.
        e - f, 2.
    }
    .expect("could not parse graph");

    match kruskal(&g) {
        Ok(kruskal_output) => {
            println!("{kruskal_output:?}");
            let mut f = File::create("kruskal.dot").expect("could not create file");
            dot::render(&kruskal_output, &mut f).expect("could not save file to output");
        }
        Err(k) => println!("more than one ({k}) component in supplied graph"),
    }
}
