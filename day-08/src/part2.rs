use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use num_integer::Integer;
use std::collections::{HashMap, HashSet};

struct AntennaGrid {
    width: usize,
    height: usize,
    antennas: HashMap<IVec2, char>,
}

impl AntennaGrid {
    fn in_bounds(&self, pos: &IVec2) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    fn antennas_by_type(&self) -> HashMap<char, Vec<IVec2>> {
        self.antennas
            .iter()
            .fold(HashMap::new(), |mut map, (k, v)| {
                map.entry(*v).or_insert_with(Vec::new).push(k.clone());
                map
            })
    }
}

fn parse(input: &str) -> AntennaGrid {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => Some((IVec2::new(x as i32, y as i32), c)),
                _ => None,
            })
        })
        .collect();

    AntennaGrid {
        width,
        height,
        antennas,
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let grid = parse(_input);

    let antinodes =
        grid.antennas_by_type()
            .iter()
            .fold(HashSet::new(), |mut set, (_, antennas)| {
                for antenna_combination in antennas.iter().combinations(2) {
                    let x_diff = antenna_combination[1].x - antenna_combination[0].x;
                    let y_diff = antenna_combination[1].y - antenna_combination[0].y;

                    let gcd = x_diff.gcd(&y_diff);

                    let x_diff = x_diff / gcd;
                    let y_diff = y_diff / gcd;

                    let mut antinode = antenna_combination[0].clone();

                    while grid.in_bounds(&antinode) {
                        set.insert(antinode);
                        antinode = antinode + IVec2::new(x_diff, y_diff);
                    }

                    let mut antinode = antenna_combination[0].clone();

                    while grid.in_bounds(&antinode) {
                        set.insert(antinode);
                        antinode = antinode - IVec2::new(x_diff, y_diff);
                    }
                }
                set
            });

    let result = antinodes.len();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
