use crate::custom_error::AocError;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter;
use std::ops::{BitAnd, BitXor, Shl, Shr};

fn step(mut n: u64) -> u64 {
    n = n.bitxor(n.shl(6));
    n = n.bitand(0xffffff);
    n = n.bitxor(n.shr(5));
    n = n.bitand(0xffffff);
    n = n.bitxor(n.shl(11));
    n = n.bitand(0xffffff);
    n
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let prices_all_monkeys = _input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|mut number| {
            iter::once((number % 10) as i8)
                .chain((0..2000).into_iter().map(|_| {
                    number = step(number);
                    (number % 10) as i8
                }))
                .collect()
        })
        .collect::<Vec<Vec<i8>>>();

    let differences_all_monkeys = prices_all_monkeys
        .iter()
        .map(|prices| {
            prices
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let differences_price_mappings_all_monkeys = differences_all_monkeys
        .iter()
        .zip(prices_all_monkeys.iter())
        .map(|(differences, prices)| {
            differences
                .iter()
                .copied()
                .tuple_windows()
                .zip(prices.iter().copied().skip(4))
                .fold(HashMap::new(), |mut acc, (diffs, price)| {
                    acc.entry(diffs).or_insert(price);
                    acc
                })
        })
        .collect_vec();

    let all_possible_difference_sequences = differences_price_mappings_all_monkeys
        .iter()
        .flat_map(|differences_price_mappings| differences_price_mappings.keys().copied())
        .collect::<HashSet<(i8, i8, i8, i8)>>();

    let result: u64 = all_possible_difference_sequences
        .iter()
        .map(|difference_sequence| {
            differences_price_mappings_all_monkeys
                .iter()
                .map(|differences_price_mappings| {
                    *differences_price_mappings
                        .get(&difference_sequence)
                        .unwrap_or(&0) as u64
                })
                .sum()
        })
        .max()
        .unwrap();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1
2
3
2024";
        assert_eq!("23", process(input)?);
        Ok(())
    }
}
