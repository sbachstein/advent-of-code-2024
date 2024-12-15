use crate::custom_error::AocError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::anychar;
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::{IResult, Parser};

fn parse(input: &str) -> IResult<&str, Vec<(Option<i32>, i32)>> {
    many0(
        many_till(
            anychar,
            alt((
                delimited(
                    tag("mul("),
                    separated_pair(complete::i32, tag(","), complete::i32),
                    tag(")"),
                )
                .map(|(v1, v2)| (Some(v1), v2)),
                tag("do()").map(|_| (None, 1)),
                tag("don't()").map(|_| (None, 0)),
            )),
        )
        .map(|(_disc, values)| values),
    )(input)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, numbers) = parse(_input).unwrap();

    let res = numbers
        .iter()
        .fold((0_i32, true), |(sum, active), item| match item {
            (None, 1) => (sum, true),
            (None, 0) => (sum, false),
            (Some(v1), v2) => {
                if active {
                    (sum + v1 * v2, active)
                } else {
                    (sum, active)
                }
            }
            _ => (sum, active),
        });

    Ok(res.0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
