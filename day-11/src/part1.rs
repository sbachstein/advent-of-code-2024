use crate::custom_error::AocError;

fn process_number(n: &u64) -> Vec<u64> {
    if *n == 0 {
        vec![1]
    } else {
        let digits = n.to_string().len() as u32;
        if digits % 2 == 0 {
            let power = 10_u64.pow(digits / 2);
            vec![n / power, n % power]
        } else {
            vec![n * 2024]
        }
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut numbers = _input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        numbers = numbers.iter().flat_map(process_number).collect::<Vec<u64>>();
    }

    Ok(numbers.len().to_string())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
