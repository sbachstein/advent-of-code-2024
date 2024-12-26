use crate::custom_error::AocError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, tuple};
use nom::Parser;
use petgraph::graphmap::GraphMap;
use petgraph::visit::{Topo, Walker};
use petgraph::Directed;
use std::collections::HashMap;
use std::io::Error;
use std::ops::Shl;

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
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let gate_configuration = parse(_input)?;

    let edges = gate_configuration
        .gates
        .iter()
        .flat_map(|(out, ((in1, in2), _))| [(*in1, *out), (*in2, *out)])
        .collect::<Vec<_>>();

    let graph: GraphMap<&str, (), Directed> = GraphMap::from_edges(&edges);

    let mut values = gate_configuration.input.clone();

    for node in Topo::new(&graph).iter(&graph) {
        if let Some(((in1, in2), gate)) = gate_configuration.gates.get(node) {
            let result = match *gate {
                GateType::And => *values.get(*in1).unwrap() && *values.get(*in2).unwrap(),
                GateType::Or => *values.get(*in1).unwrap() || *values.get(*in2).unwrap(),
                GateType::Xor => *values.get(*in1).unwrap() ^ *values.get(*in2).unwrap(),
            };
            values.insert(node, result);
        }
    }

    let mut z = values
        .into_iter()
        .filter(|(node, _value)| node.starts_with("z"))
        .collect::<Vec<_>>();
    z.sort_by_key(|(node, _value)| *node);
    let result = z.iter().rev().fold(0u64, |mut acc, (_, value)| {
        acc = acc.shl(1);
        if *value {
            acc += 1;
        }
        acc
    });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> miette::Result<()> {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_2() -> miette::Result<()> {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!("2024", process(input)?);
        Ok(())
    }
}
