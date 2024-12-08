use crate::custom_error::AocError;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Equation {
    fn is_valid(&self) -> bool {
        (1..=self.operands.len() - 1)
            .map(|_| vec![Operator::Add, Operator::Multiply].into_iter())
            .multi_cartesian_product()
            .map(|operators| {
                let mut operand_iter = self.operands.iter();
                let mut result = operand_iter.next().unwrap().clone();
                for operand in operators {
                    match operand {
                        Operator::Add => {
                            result += operand_iter.next().unwrap();
                        }
                        Operator::Multiply => {
                            result *= operand_iter.next().unwrap();
                        }
                    }
                }
                result == self.result
            })
            .any(|x| x)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list0(newline, separated_pair(complete::u64, tag(": "), separated_list0(tag(" "), complete::u64)))(input)
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (_, parsed) = parse(_input).unwrap();

    let equations = parsed.into_iter().map(|eq| {
        Equation {
            result: eq.0,
            operands: eq.1,
        }
    }).collect_vec();

    dbg!(&equations);

    let result = equations.iter().filter_map(|eq| {
        match eq.is_valid() {
            true => Some(eq.result),
            false => None,
        }
    }).sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
