use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, AocError> {
    let res = _input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse::<i32>().unwrap())
                .tuple_windows()
                .fold(
                    (None, true),
                    |acc: (Option<bool>, bool), window: (i32, i32)| {
                        let abs_diff = window.1.abs_diff(window.0);
                        if !(1..=3).contains(&abs_diff) {
                            (None, false)
                        } else {
                            match acc {
                                (inc, false) => (inc, false),
                                (None, true) => {
                                    if window.0 < window.1 {
                                        (Some(true), true)
                                    } else {
                                        (Some(false), true)
                                    }
                                }
                                (Some(inc), true) => {
                                    if inc {
                                        (Some(inc), window.1 - window.0 > 0)
                                    } else {
                                        (Some(inc), window.0 - window.1 > 0)
                                    }
                                }
                            }
                        }
                    },
                )
                .1
        })
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
