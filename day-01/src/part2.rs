use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::multispace1;
use nom::multi::fold_many0;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::collections::HashMap;

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    fold_many0(
        terminated(separated_pair(complete::u32, multispace1, complete::u32), tag("\n")),
        || {(Vec::new(), Vec::new())},
        |mut acc, item| {
            acc.0.push(item.0);
            acc.1.push(item.1);
            acc
        }
    )(input)
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (_, lists) = parse_input(_input).unwrap();

    let frequencies = lists.1.iter().fold(HashMap::new(), |mut frequency, item| {
        dbg!(&item);
        frequency.entry(item).and_modify(|e| *e += 1).or_insert(1);
        frequency
    });

    dbg!(&frequencies);

    let res = lists.0.iter().filter_map(
        |item| {
            frequencies.get(item).map(|freq| item * freq)
        }).sum::<u32>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
