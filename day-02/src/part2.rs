use itertools::Itertools;
use crate::custom_error::AocError;


fn find_faulty_index(numbers: &[i32]) -> Option<usize> {
    let diffs = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    let direction = diffs.iter()
        .map(|diff| diff.signum()).counts()
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k).unwrap();

    diffs
        .iter()
        .map(|diff| {
            diff.signum() == direction && (1..=3).contains(&diff.abs())
        })
        .position(|good| !good)
}
fn is_report_safe(report: &str) -> bool {
    let numbers = report
        .split(' ')
        .map(|number| number.parse::<i32>().unwrap())
        .collect_vec();


    let bad_index = find_faulty_index(&numbers);

    match bad_index {
        None => true,
        Some(index) => {
            let mut first_removed = numbers.clone();
            first_removed.remove(index);
            let mut second_removed = numbers.clone();
            second_removed.remove(index + 1);
            find_faulty_index(&first_removed).is_none()
                || find_faulty_index(&second_removed).is_none()
        }
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> Result<String, AocError> {
    let res = _input
        .lines()
        .map(
            is_report_safe
        )
        .filter(|&safe| safe)
        .count();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
