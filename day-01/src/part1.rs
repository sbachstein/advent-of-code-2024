use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::multispace1;
use nom::multi::fold_many1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, multispace1, complete::u32),
            tag("\n"),
        ),
        || (Vec::new(), Vec::new()),
        |mut acc, item| {
            acc.0.push(item.0);
            acc.1.push(item.1);
            acc
        },
    )(input)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, mut lists) = parse_input(_input).unwrap();

    lists.0.sort();
    lists.1.sort();

    dbg!(&lists);

    let res = lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|(v1, v2)| (*v1 as i32 - *v2 as i32).abs())
        .sum::<i32>();

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
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
