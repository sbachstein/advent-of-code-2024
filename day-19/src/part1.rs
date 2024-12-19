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
    ty = "UnboundCache<String, bool>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{}", design) }"#
)]
fn match_design(design: &str, patterns: &Vec<&str>) -> bool {
    if design.is_empty() {
        true
    } else {
        patterns.iter().any(|&p| {
            design.len() >= p.len()
                && (design[..p.len()] == *p)
                && match_design(&design[p.len()..], patterns)
        })
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (patterns, designs) = parse(_input)?;

    let result = designs
        .into_iter()
        .filter(|&design| match_design(design, &patterns))
        .count();

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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
