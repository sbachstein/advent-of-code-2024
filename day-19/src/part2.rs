use crate::custom_error::AocError;
use cached::proc_macro::cached;
use cached::UnboundCache;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use std::io::Error;

fn parse(input: &str) -> Result<(Vec<&str>, Vec<&str>), AocError> {
    let (_, result) = separated_pair(
        separated_list0(tag(", "), alphanumeric1::<&str, ()>),
        tag("\n\n"),
        separated_list0(tag("\n"), alphanumeric1::<&str, ()>),
    )(input)
    .map_err(|_| Error::other("Invalid input"))?;

    Ok((result.0, result.1))
}

#[cached(
    ty = "UnboundCache<String, u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{}", design) }"#
)]
fn num_arrangements(design: &str, patterns: &Vec<&str>) -> u64 {
    if design.is_empty() {
        1
    } else {
        patterns
            .iter()
            .map(|&p| {
                if design.len() >= p.len() && (design[..p.len()] == *p) {
                    num_arrangements(&design[p.len()..], patterns)
                } else {
                    0
                }
            })
            .sum()
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (patterns, designs) = parse(_input)?;

    let result = designs
        .into_iter()
        .map(|design| num_arrangements(design, &patterns))
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("16", process(input)?);
        Ok(())
    }
}
