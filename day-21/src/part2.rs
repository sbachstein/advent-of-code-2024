use crate::custom_error::AocError;
use cached::proc_macro::cached;
use cached::UnboundCache;
use glam::IVec2;
use itertools::Itertools;
use phf::{phf_map, Map};
use std::io::Error;
use std::iter;
use std::num::ParseIntError;

static DIGITMAP: Map<char, IVec2> = phf_map! {
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

static DIRMAP: Map<char, IVec2> = phf_map! {
    '^' => IVec2::new(1, 0),
    'v' => IVec2::new(1, 1),
    '<' => IVec2::new(0, 1),
    '>' => IVec2::new(2, 1),
    'A' => IVec2::new(2, 0),
};

#[cached(
    ty = "UnboundCache<String, usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{}, {}, Level {}", from, to, levels) }"#
)]
fn shortest_paths(
    from: char,
    to: char,
    keymap: &Map<char, IVec2>,
    forbidden: IVec2,
    levels: u8,
) -> usize {
    if levels == 0 {
        return 1
    }

    let start_coords = keymap.get(&from).unwrap();
    let target_coords = keymap.get(&to).unwrap();

    let diff = target_coords - start_coords;

    let horizontal_moves = if diff.x > 0 {
        iter::repeat_n('>', diff.x as usize)
    } else {
        iter::repeat_n('<', -diff.x as usize)
    };

    let vertical_moves = if diff.y > 0 {
        iter::repeat_n('v', diff.y as usize)
    } else {
        iter::repeat_n('^', -diff.y as usize)
    };

    horizontal_moves
        .chain(vertical_moves)
        .permutations((diff.x.abs() + diff.y.abs()) as usize)
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
            iter::once('A')
                .chain(candidate)
                .chain(iter::once('A'))
                .tuple_windows()
                .map(|(from, to)| shortest_paths(from, to, &DIRMAP, IVec2::new(0, 0), levels - 1))
                .sum()
        })
        .min().unwrap()
}

#[tracing::instrument]
pub fn process(_input: &str, levels: u8) -> miette::Result<String, AocError> {
    let results = _input
        .lines()
        .map(|line| {
            let number: usize = line[..line.len() - 1].parse::<usize>()?;

            let shortest_length = iter::once('A')
                .chain(line.chars())
                .tuple_windows()
                .map(|(from, to)| shortest_paths(from, to, &DIGITMAP, IVec2::new(0, 3), levels))
                .sum::<usize>();

            Ok::<usize, ParseIntError>(shortest_length * number)
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
        assert_eq!("126384", process(input, 3)?);
        Ok(())
    }
}
