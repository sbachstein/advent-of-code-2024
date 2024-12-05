use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::anychar;
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::{IResult, Parser};

fn parse(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many0(
        many_till(anychar, delimited(tag("mul("), separated_pair(complete::i32, tag(","), complete::i32), tag(")")))
            .map(|(_disc, values)| values)
    )(input)
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (_, numbers) = parse(_input).unwrap();

    Ok(numbers.iter().map(|(m1, m2)| m1*m2).sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let mut input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
