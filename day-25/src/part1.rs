use crate::custom_error::AocError;
use itertools::Itertools;
use std::io::Error;

fn parse(input: &str) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), AocError> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for object_str in input.split("\n\n") {
        let mut levels = object_str.lines().peekable();
        let is_key = match levels.peek() {
            Some(&".....") => Ok(true),
            Some(&"#####") => Ok(false),
            _ => Err(Error::other("Invalid input")),
        }?;

        let level_folder = |mut acc, (level, line)| {
            line.chars().enumerate().for_each(|(pos, c)| {
                if c == '#' {
                    acc[pos] = level as u8;
                }
            });
            acc
        };

        if is_key {
            let key = levels.rev().enumerate().fold(vec![0u8; 5], level_folder);
            keys.push(key);
        } else {
            let lock = levels.enumerate().fold(vec![0u8; 5], level_folder);
            locks.push(lock);
        }
    }

    Ok((locks, keys))
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (locks, keys) = parse(_input)?;

    let result = locks
        .into_iter()
        .cartesian_product(keys.into_iter())
        .filter(|(lock, key)| {
            lock.iter()
                .zip(key.iter())
                .all(|(&lock_level, &key_level)| lock_level + key_level <= 5)
        })
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
