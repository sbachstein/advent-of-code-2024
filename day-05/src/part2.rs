use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;

struct PageOrdering {
    order_rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse(input: &str) -> IResult<&str, PageOrdering> {
    separated_pair(
        separated_list0(
            tag("\n"),
            separated_pair(complete::u32, tag("|"), complete::u32),
        ),
        tag("\n\n"),
        separated_list0(tag("\n"), separated_list0(tag(","), complete::u32)),
    )(input)
    .map(|(input, (order_rules, updates))| {
        (
            input,
            PageOrdering {
                order_rules,
                updates,
            },
        )
    })
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, mut page_ordering) = parse(_input.trim()).unwrap();

    let result = page_ordering
        .updates
        .iter_mut()
        .filter(|update| {
            match page_ordering
                .order_rules
                .iter()
                .map(|(predecessor, successor)| {
                    match (
                        update.iter().position(|item| item == predecessor),
                        update.iter().position(|item| item == successor),
                    ) {
                        (Some(p1), Some(p2)) if p1 < p2 => true,
                        (None, _) => true,
                        (_, None) => true,
                        _ => false,
                    }
                })
                .all(|valid| valid)
            {
                true => false,
                false => true,
            }
        })
        .filter_map(|update| {
            update.sort_by(|a, b| {
                if page_ordering.order_rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if page_ordering.order_rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update.get((update.len() - 1) / 2)
        })
        .sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
