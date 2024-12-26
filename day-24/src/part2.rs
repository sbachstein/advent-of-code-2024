use crate::custom_error::AocError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, tuple};
use nom::Parser;
use petgraph::dot::Dot;
use petgraph::graphmap::GraphMap;
use petgraph::visit::Walker;
use petgraph::Directed;
use std::collections::HashMap;
use std::io::Error;

#[derive(Debug)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct GateConfiguration<'a> {
    input: HashMap<&'a str, bool>,
    gates: HashMap<&'a str, ((&'a str, &'a str), GateType)>,
}

fn parse(input: &str) -> Result<GateConfiguration, AocError> {
    let (_, parsed) = separated_pair(
        separated_list0(
            newline::<&str, ()>,
            separated_pair(alphanumeric1, tag(": "), alt((tag("0"), tag("1")))).map(
                |(name, value)| {
                    (
                        name,
                        match value {
                            "1" => true,
                            _ => false,
                        },
                    )
                },
            ),
        )
        .map(|inputs| inputs.into_iter().collect::<HashMap<_, _>>()),
        tag("\n\n"),
        separated_list0(
            newline,
            separated_pair(
                tuple((
                    alphanumeric1,
                    alt((tag(" AND "), tag(" OR "), tag(" XOR "))),
                    alphanumeric1,
                ))
                .map(|(name1, gate, name2)| {
                    (
                        (name1, name2),
                        match gate {
                            " AND " => GateType::And,
                            " OR " => GateType::Or,
                            _ => GateType::Xor,
                        },
                    )
                }),
                tag(" -> "),
                alphanumeric1,
            ),
        )
        .map(|gates| {
            gates
                .into_iter()
                .map(|(gate, output)| (output, gate))
                .collect::<HashMap<_, _>>()
        }),
    )(input)
    .map_err(|_| Error::other("Invalid input"))?;

    Ok(GateConfiguration {
        input: parsed.0,
        gates: parsed.1,
    })
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<(), AocError> {
    let gate_configuration = parse(_input)?;

    let edges = gate_configuration
        .gates
        .iter()
        .flat_map(|(out, ((in1, in2), gate))| {
            let gate_str = match gate {
                GateType::And => "AND",
                GateType::Or => "OR",
                GateType::Xor => "XOR",
            }
            .to_string();
            [(*in1, *out, gate_str.clone()), (*in2, *out, gate_str)]
        })
        .collect::<Vec<_>>();

    let graph: GraphMap<&str, String, Directed> = GraphMap::from_edges(&edges);

    println!("{:?}", Dot::new(&graph));

    Ok(())

    // Solution involved manually checking the gate graph for consistency with a full adder
    // and marking gates to be swapped.
}
