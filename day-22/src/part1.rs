use crate::custom_error::AocError;
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
    let result = _input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|mut number| {
            for _ in 0..2000 {
                number = step(number);
            }
            number
        })
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(input)?);
        Ok(())
    }
}
