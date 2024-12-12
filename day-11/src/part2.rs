use cached::proc_macro::cached;
use crate::custom_error::AocError;

#[cached]
fn process_number(number: u64, times: u8) -> u64 {
    if times == 0 {
        1
    } else {
        if number == 0 {
            process_number(1, times - 1)
        } else {
            let digits = number.to_string().len() as u32;
            if digits % 2 == 0 {
                let power = 10_u64.pow(digits / 2);
                process_number(number / power, times - 1)
                    + process_number(number % power, times - 1)
            } else {
                process_number(number * 2024, times - 1)
            }
        }
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let numbers = _input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let result = numbers.iter().map(|n| process_number(*n, 75)).sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", crate::part1::process(input)?);
        Ok(())
    }
}

