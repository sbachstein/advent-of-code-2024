use crate::custom_error::AocError;
use crate::custom_error::AocError::IoError;
use glam::IVec2;
use itertools::Itertools;
use phf::{phf_map, Map};
use std::io::Error;
use std::iter;
use std::num::ParseIntError;

static DIGITPAD: Map<char, IVec2> = phf_map! {
    'A' => IVec2::new(2, 3),
    '0' => IVec2::new(1, 3),
    '1' => IVec2::new(0, 2),
    '2' => IVec2::new(1, 2),
    '3' => IVec2::new(2, 2),
    '4' => IVec2::new(0, 1),
    '5' => IVec2::new(1, 1),
    '6' => IVec2::new(2, 1),
    '7' => IVec2::new(0, 0),
    '8' => IVec2::new(1, 0),
    '9' => IVec2::new(2, 0),
};

static DIRPAD: Map<char, IVec2> = phf_map! {
    '^' => IVec2::new(1, 0),
    'v' => IVec2::new(1, 1),
    '<' => IVec2::new(0, 1),
    '>' => IVec2::new(2, 1),
    'A' => IVec2::new(2, 0),
};

enum DigitButton {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
}

impl TryFrom<char> for DigitButton {
    type Error = AocError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(DigitButton::One),
            '2' => Ok(DigitButton::Two),
            '3' => Ok(DigitButton::Three),
            '4' => Ok(DigitButton::Four),
            '5' => Ok(DigitButton::Five),
            '6' => Ok(DigitButton::Six),
            '7' => Ok(DigitButton::Seven),
            '8' => Ok(DigitButton::Eight),
            '9' => Ok(DigitButton::Nine),
            '0' => Ok(DigitButton::Zero),
            'A' => Ok(DigitButton::A),
            _ => Err(IoError(Error::other(format!(
                "Invalid character {}",
                value
            )))),
        }
    }
}

fn shortest_path(
    button_sequence: &Vec<char>,
    keymap: &Map<char, IVec2>,
    forbidden: IVec2,
    levels: u8,
) -> (Vec<char>, usize) {
    iter::once(&'A')
        .chain(button_sequence)
        .tuple_windows()
        .map(|(start, target)| {
            let start_coords = keymap.get(&start).unwrap();
            let target_coords = keymap.get(&target).unwrap();

            let diff = target_coords - start_coords;

            let horizontal_moves = if diff.x > 0 {
                iter::repeat_n('>', diff.x as usize)
            } else {
                iter::repeat_n('<', -diff.x as usize)
            }
            .collect_vec();

            let vertical_moves = if diff.y > 0 {
                iter::repeat_n('v', diff.y as usize)
            } else {
                iter::repeat_n('^', -diff.y as usize)
            }
            .collect_vec();

            let complete_moves = horizontal_moves
                .clone()
                .into_iter()
                .chain(vertical_moves.clone())
                .collect_vec();

            let candidates = complete_moves
                .clone()
                .into_iter()
                .permutations(complete_moves.len())
                .unique()
                .filter(|candidate| {
                    let mut coords = start_coords.clone();
                    for &c in candidate {
                        coords += match c {
                            '^' => IVec2::new(0, -1),
                            'v' => IVec2::new(0, 1),
                            '>' => IVec2::new(1, 0),
                            '<' => IVec2::new(-1, 0),
                            _ => IVec2::new(0, 0),
                        };
                        if coords == forbidden {
                            return false;
                        }
                    }
                    true
                })
                .map(|candidate| {
                    let mut final_candidate = candidate.clone();
                    final_candidate.push('A');
                    final_candidate
                })
                .collect_vec();

            if levels == 0 {
                (candidates.get(0).unwrap().clone(), candidates[0].len())
            } else {
                candidates
                    .into_iter()
                    .map(|candidate| {
                        shortest_path(&candidate, &DIRPAD, IVec2::new(0, 0), levels - 1)
                    })
                    .min_by_key(|(_seq, len)| *len)
                    .unwrap()
            }
        })
        .fold((Vec::new(), 0), |mut acc, (mut seq, len)| {
            acc.0.append(&mut seq);
            acc.1 += len;
            acc
        })
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let results = _input
        .lines()
        .map(|line| {
            let number: usize = line[..line.len() - 1].parse::<usize>()?;

            let shortest_paths =
                shortest_path(&line.chars().collect(), &DIGITPAD, IVec2::new(0, 3), 2);

            println!("{}", shortest_paths.0.iter().format(""));

            let lenght_shortest_seq = shortest_paths.1;

            Ok::<usize, ParseIntError>(dbg!(lenght_shortest_seq) * dbg!(number))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::other("Invalid input"))?;

    let sum = results.iter().sum::<usize>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A
";
        assert_eq!("126384", process(input)?);
        Ok(())
    }
}
