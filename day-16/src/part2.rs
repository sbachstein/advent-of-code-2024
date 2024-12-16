use crate::custom_error::AocError;
use glam::IVec2;
use pathfinding::prelude::astar_bag;
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
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let maze = parse(_input)?;

    let shortest_path = astar_bag(
        &(maze.start, Direction::East),
        |(pos, dir)| {
            // Rotate
            let mut rotations: Vec<((IVec2, Direction), usize)> = DIRECTIONS
                .iter()
                .copied()
                .filter(|new_dir| new_dir != dir)
                .map(|new_dir| ((pos.clone(), new_dir), 1000))
                .collect();

            // Take step
            let step = pos + IVec2::from(*dir);
            if !maze.walls.contains(&step) {
                rotations.push(((step, *dir), 1));
            }

            rotations
        },
        |_| 0,
        |(pos, _dir)| *pos == maze.end,
    );

    let paths = shortest_path.ok_or(Error::other("No path found"))?.0;

    let spots = paths
        .fold(HashSet::new(), |mut map, path| {
            path.iter().for_each(|(pos, _dir)| {
                map.insert(*pos);
            });
            map
        })
        .len();

    Ok(spots.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("45", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("64", process(input)?);
        Ok(())
    }
}
