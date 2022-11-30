#![feature(let_chains)]

use std::cmp::{Eq, Ord, PartialEq, PartialOrd, Reverse};

type Vertex = usize;

#[derive(Debug, Clone, Copy, Eq)]
struct Edge {
    u: Vertex,
    v: Vertex,
    w: isize,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.w.cmp(&other.w)
    }
}

#[derive(Debug, Clone)]
struct Graph<const N: usize> {
    adjacency_list: [Vec<(Vertex, isize)>; N],
}

impl<const N: usize> Graph<N> {
    pub fn from_adjacency_list(adjacency_list: [Vec<(Vertex, isize)>; N]) -> Self {
        Self { adjacency_list }
    }

    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.adjacency_list
            .iter()
            .enumerate()
            .flat_map(|(u, l)| l.iter().map(move |&(v, w)| Edge { u, v, w }))
    }
}

fn bfs<const N: usize>(graph: &Graph<N>, s: Vertex) -> ([isize; N], [Option<Vertex>; N]) {
    use std::collections::VecDeque;
    let mut d = [-1; N];
    let mut pi = [None; N];

    let mut q = VecDeque::with_capacity(N);
    d[s] = 0;

    q.push_back(s);

    while let Some(u) = q.pop_front() {
        for &(v, _) in &graph.adjacency_list[u] {
            if d[v] < 0 {
                d[v] = d[u] + 1;
                pi[v] = Some(u);
                q.push_back(v);
            }
        }
    }

    (d, pi)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Grey,
    Black,
}

#[allow(clippy::type_complexity)]
fn dfs<const N: usize>(
    graph: &Graph<N>,
) -> (
    [usize; N],
    [usize; N],
    [Option<Vertex>; N],
    Option<Vec<Vertex>>,
) {
    let mut time: usize = 0;
    let mut color = [Color::White; N];
    let mut d = [0; N];
    let mut f = [0; N];
    let mut pi = [None; N];
    let mut topological_stack = Some(Vec::new());

    for u in 0..N {
        if color[u] == Color::White {
            dfs_visit(
                graph,
                u,
                &mut time,
                &mut color,
                &mut d,
                &mut f,
                &mut pi,
                &mut topological_stack,
            );
        }
    }

    (d, f, pi, topological_stack)
}

#[allow(clippy::too_many_arguments)]
fn dfs_visit<const N: usize>(
    graph: &Graph<N>,
    u: Vertex,
    time: &mut usize,
    color: &mut [Color; N],
    d: &mut [usize; N],
    f: &mut [usize; N],
    pi: &mut [Option<Vertex>; N],
    topological_stack: &mut Option<Vec<Vertex>>,
) {
    color[u] = Color::Grey;
    *time += 1;
    d[u] = *time;
    for &(v, _) in &graph.adjacency_list[u] {
        if color[v] == Color::White {
            pi[v] = Some(u);
            dfs_visit(graph, v, time, color, d, f, pi, topological_stack);
        } else if color[v] == Color::Grey {
            topological_stack.take();
        }
    }

    *time += 1;
    f[u] = *time;
    color[u] = Color::Black;
    if let Some(s) = topological_stack {
        s.push(u);
    }
}

fn kruskal<const N: usize>(graph: &Graph<N>) -> (usize, Vec<Edge>) {
    use std::collections::BinaryHeap;

    let mut pi = [0; N];
    let mut s = [1; N];
    let mut k = N;
    let mut a = Vec::new();
    let mut q = graph.edges().map(Reverse).collect::<BinaryHeap<_>>();

    for v in 0..N {
        make_set(v, &mut pi, &mut s);
    }

    while k > 1 && let Some(Reverse(e)) = q.pop(){
        let x = find_set(e.u, &mut pi); let y = find_set(e.v, &mut pi);
        if x != y {
            k -= 1;
            a.push(e);
            union_set(x,y, &mut pi, &mut s);
            
        }
    }

    (k, a)
}
fn make_set<const N: usize>(v: Vertex, pi: &mut [Vertex; N], s: &mut [usize; N]) {
    pi[v] = v;
    s[v] = 1;
}

fn find_set<const N: usize>(v: Vertex, pi: &mut [Vertex; N]) -> Vertex {
    if pi[v] != v {
        pi[v] = find_set(pi[v], pi);
    }
    pi[v]
}

fn union_set<const N: usize>(x: Vertex, y: Vertex, pi: &mut [Vertex; N], s: &mut [usize; N]) {
    if s[x] < s[y] {
        pi[x] = y;
        s[y] += s[x];
    }  else {
        pi[y] = x;
        s[x] += s[y];
    } 
}

//TODO: Prim, Dijkstra, DAGshP

fn qbf<const N: usize>(graph: &Graph<N>, s: Vertex) -> (Option<Vertex>, [isize; N], [Option<Vertex>; N]) {
    use std::collections::VecDeque;    
    
    let mut d = [-1; N];
    let mut pi = [None; N];
    let mut e = [0; N];
    let mut inq = [false; N];
    let mut q = VecDeque::with_capacity(N);
    
    d[s] = 0; 
    q.push_back(s);
    inq[s] = true;
    
    while let Some(u) = q.pop_front() {
        inq[u] = false;
        
        for &(v, w) in &graph.adjacency_list[u] {
            if d[v] == -1 || d[v] > d[u] + w {
                d[v] = d[u] + w;
                pi[v] = Some(u);
                e[v] += 1;
                
                if e[v] < N  {
                    if !inq[v] {
                        q.push_back(v);
                        inq[v] = true;
                    }
                } else {
                    return (Some(find_neg_cycle(v, &pi)), d, pi);
                }
            }
        }
    }
    
    (None, d, pi)
    
}

fn find_neg_cycle<const N: usize>(mut v: Vertex, pi: &[Option<Vertex>; N]) -> Vertex {
    let mut b = [false; N];
    
    b[v] = true; v = pi[v].expect("in case of a negative cycle v should always have a parent");
    
    while !b[v] {
        b[v] = true; 
        v = pi[v].expect("in case of a negative cycle v should always have a parent");
    }
    
    v
}


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
    
    let g3 = Graph::from_adjacency_list([vec![(1,0), (3,2), (4,2)], vec![(0,0),(2,1),(4,1),(5,2)], vec![(1,1),(5,3)], vec![(0,2),(4,0)], vec![(0,2),(1,1),(3,0),(5,2)], vec![(1,2),(2,3),(4,2)]]);
    
    println!("{:?}", kruskal(&g3));
    
    let g4 = Graph::from_adjacency_list([vec![(1,3), (2,1)], vec![(3, -2)], vec![(1, -1)], vec![(2,3)]]);
    
    println!("{:?}", qbf(&g4, 0));
}
