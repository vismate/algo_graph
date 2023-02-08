use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, satisfy},
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::graph::Vertex;

#[derive(Debug, Clone)]
pub struct Statement {
    node: Vertex,
    is_directed: bool,
    neighbours: Vec<(Vertex, isize)>,
}

fn identifier(input: &str) -> IResult<&str, Vertex> {
    map(satisfy(|c| c.is_ascii_alphanumeric()), |c| {
        c as usize
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

fn neighbours(input: &str) -> IResult<&str, Vec<(Vertex, isize)>> {
    let sign = map(alt((char('-'), char('+'))), |c| match c {
        '-' => -1,
        _ => 1,
    });
    let number = map(
        tuple((opt(sign), digit1)),
        |(sign, val): (Option<isize>, &str)| {
            let num = val.parse::<isize>().expect("weight cannot fit in a isize");
            num * sign.unwrap_or(1)
        },
    );
    let weight = preceded(char(','), number);
    let neighbour = map(tuple((identifier, opt(weight))), |(idx, weight)| {
        (idx, weight.unwrap_or(1))
    });
    let list = separated_list0(char(';'), neighbour);
    terminated(list, char('.'))(input)
}

pub fn statements(input: &str) -> IResult<&str, Vec<Statement>> {
    let entry = tuple((identifier, edge_type, neighbours));

    let statement = map(entry, |(node, is_directed, neighbours)| Statement {
        node,
        is_directed,
        neighbours,
    });

    many0(statement)(input)
}

pub fn create_graph<const N: usize>(
    statements: Vec<Statement>,
) -> Result<[Vec<(Vertex, isize)>; N], String>
where
    [Vec<(Vertex, isize)>; N]: Default,
{
    fn insert_uniq_sorted(vec: &mut Vec<(Vertex, isize)>, val: (Vertex, isize)) {
        if let Err(pos) = vec.binary_search_by_key(&val.0, |&(idx, _)| idx) {
            vec.insert(pos, val);
        }
    }

    let mut adjacency_list: [Vec<(Vertex, isize)>; N] = Default::default();

    for statement in statements {
        if statement.node >= N {
            return Err(format!(
                "node {} does not fit in graph of size {N}",
                statement.node
            ));
        }
        for val @ (neighbour, weight) in statement.neighbours {
            if neighbour >= N {
                return Err(format!(
                    "node {neighbour} does not fit in graph of size {N}"
                ));
            }

            if statement.node == neighbour {
                return Err("loop edges are not allowed".into());
            }

            insert_uniq_sorted(&mut adjacency_list[statement.node], val);

            if !statement.is_directed {
                insert_uniq_sorted(&mut adjacency_list[neighbour], (statement.node, weight));
            }
        }
    }

    Ok(adjacency_list)
}

#[macro_export]
macro_rules! graph {
    (Nodes:$N:expr; $($t:tt)*) => {
        {
            const _: () = assert!($N <= 26);

            let input: String = stringify!($($t)*)
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            match $crate::parse_graph::statements(&input) {
                Ok(("", res)) => {
                    $crate::parse_graph::create_graph::<$N>(res)
                },
                Err(e) => Err(e.to_string()),
                _ => Err("syntax error".into())
            }.map(|l| $crate::graph::Graph::from_adjacency_list(l))
        }
    };
}

pub use graph;
