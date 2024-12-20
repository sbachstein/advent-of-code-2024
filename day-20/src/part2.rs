use crate::custom_error::AocError;
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::collections::HashSet;
use std::io::Error;

struct Maze {
    walls: HashSet<IVec2>,
    start: IVec2,
    end: IVec2,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl From<Direction> for IVec2 {
    fn from(dir: Direction) -> IVec2 {
        match dir {
            Direction::North => IVec2::new(0, -1),
            Direction::South => IVec2::new(0, 1),
            Direction::West => IVec2::new(-1, 0),
            Direction::East => IVec2::new(1, 0),
        }
    }
}

fn parse(input: &str) -> Result<Maze, AocError> {
    let mut walls = HashSet::new();
    let mut start: Option<IVec2> = None;
    let mut end: Option<IVec2> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert(IVec2::new(x as i32, y as i32));
                }
                'S' => {
                    start = Some(IVec2::new(x as i32, y as i32));
                }
                'E' => {
                    end = Some(IVec2::new(x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    Ok(Maze {
        walls,
        start: start.ok_or(Error::other("No starting position"))?,
        end: end.ok_or(Error::other("No starting position"))?,
    })
}

#[tracing::instrument]
pub fn process(_input: &str, minimum_saving: usize) -> miette::Result<String, AocError> {
    let maze = parse(_input)?;

    let normal_path = dijkstra(
        &maze.start,
        |pos| {
            DIRECTIONS
                .iter()
                .map(|&dir| (pos + IVec2::from(dir), 1))
                .filter(|(pos, _cost)| !maze.walls.contains(pos))
                .collect::<Vec<(IVec2, usize)>>()
        },
        |pos| *pos == maze.end,
    )
    .ok_or(Error::other("No path found"))?;

    let normal_time = normal_path.1;

    let cheated_path_times = normal_path
        .0
        .iter()
        .enumerate()
        .flat_map(|(previous_steps, &pos)| {
            normal_path.0.iter().enumerate().filter_map(
                move |(back_on_track_steps, back_on_track_pos)| {
                    let manhattan_distance = ((back_on_track_pos - pos).x.abs()
                        + (back_on_track_pos - pos).y.abs())
                        as usize;
                    if manhattan_distance <= 20 {
                        Some(
                            normal_time - back_on_track_steps + previous_steps + manhattan_distance,
                        )
                    } else {
                        None
                    }
                },
            )
        })
        .collect::<Vec<usize>>();

    let num_cheated_paths_with_minimum_savings = cheated_path_times
        .iter()
        .filter(|&&time| time <= normal_time - minimum_saving)
        .count();

    Ok(num_cheated_paths_with_minimum_savings.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("285", process(input, 50)?);
        Ok(())
    }
}
