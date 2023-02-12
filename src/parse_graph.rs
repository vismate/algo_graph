use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, satisfy},
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::graph::{Graph, Vertex};

#[derive(Debug, Clone)]
struct Statement {
    node: Vertex,
    is_directed: bool,
    adjacent_nodes: Vec<(Vertex, isize)>,
}

fn vertex(input: &str) -> IResult<&str, Vertex> {
    map(satisfy(|c| c.is_ascii_alphanumeric()), |c| {
        c as Vertex
            - if c < 'A' {
                48
            } else if c < 'a' {
                65
            } else {
                97
            }
    })(input)
}

fn edge_type(input: &str) -> IResult<&str, bool> {
    let typ = alt((tag("->"), tag("-")));
    map(typ, |pat| pat == "->")(input)
}

fn signed_number(input: &str) -> IResult<&str, isize> {
    let sign = map(alt((char('-'), char('+'))), |c| match c {
        '-' => -1,
        _ => 1,
    });

    map(
        tuple((opt(sign), digit1)),
        |(sign, val): (Option<isize>, &str)| {
            let num = val.parse::<isize>().expect("weight cannot fit in a isize");
            num * sign.unwrap_or(1)
        },
    )(input)
}

fn adjacent_nodes(input: &str) -> IResult<&str, Vec<(Vertex, isize)>> {
    let weight = preceded(char(','), signed_number);
    let neighbour = map(tuple((vertex, opt(weight))), |(idx, weight)| {
        (idx, weight.unwrap_or(1))
    });
    let list = separated_list0(char(';'), neighbour);
    terminated(list, char('.'))(input)
}

fn statements(input: &str) -> IResult<&str, Vec<Statement>> {
    let statement = tuple((vertex, edge_type, adjacent_nodes));

    let statement = map(statement, |(node, is_directed, neighbours)| Statement {
        node,
        is_directed,
        adjacent_nodes: neighbours,
    });

    many0(statement)(input)
}

fn create_graph<const N: usize>(statements: &[Statement]) -> Result<Graph<N>, String>
where
    [Vec<(Vertex, isize)>; N]: Default,
{
    let mut adjacency_list: [Vec<(Vertex, isize)>; N] = Default::default();

    macro_rules! insert_uniq_sorted {
        ($node: expr, $val:expr) => {
            let l = &mut adjacency_list[$node];

            match l.binary_search_by_key(&$val.0, |&(idx, _)| idx) {
                Err(pos) => {
                    l.insert(pos, $val);
                }
                Ok(pos) if l[pos].1 != $val.1 => {
                    return Err(format!(
                        "node {} already has a edge to node {} with a different weight (multi edges are not allowed)",
                        $node, $val.0
                    ));
                }
                _ => {}
            }
        };
    }

    for statement in statements {
        if statement.node >= N {
            return Err(format!(
                "node {} does not fit in graph of size {N}",
                statement.node
            ));
        }
        for val @ &(adjacent, weight) in &statement.adjacent_nodes {
            if adjacent >= N {
                return Err(format!("node {adjacent} does not fit in graph of size {N}"));
            }

            if statement.node == adjacent {
                return Err("loop edges are not allowed".into());
            }

            insert_uniq_sorted!(statement.node, *val);

            if !statement.is_directed {
                insert_uniq_sorted!(adjacent, (statement.node, weight));
            }
        }
    }

    Ok(Graph { adjacency_list })
}

pub fn parse_graph<const N: usize>(input: &str) -> Result<Graph<N>, String>
where
    [Vec<(Vertex, isize)>; N]: Default,
{
    if N > 26 {
        return Err("number of nodes must be less or equal to 26".into());
    }

    // TODO: refactor so that this is not necessary
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    match statements(&input) {
        Ok(("", res)) => create_graph(&res),
        Err(e) => Err(e.to_string()),
        _ => Err("syntax error".into()),
    }
}

#[macro_export]
macro_rules! graph {
    (Nodes:$N:expr; $($t:tt)*) => {
        {
            // parse_graph check N again in runtime, but it is still useful to fail the compilation instead of producing a runtime error
            const _: () = assert!($N <= 26);
            $crate::parse_graph::parse_graph::<$N>(stringify!($($t)*))
        }
    };
}

pub use graph;
