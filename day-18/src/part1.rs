use crate::custom_error::AocError;
use glam::IVec2;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use pathfinding::prelude::dijkstra;
use std::io::Error;

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

fn parse(input: &str) -> Result<Vec<IVec2>, AocError> {
    let (_, result) = separated_list0(
        tag("\n"),
        separated_pair(
            complete::i32::<&str, ()>,
            tag(","),
            complete::i32::<&str, ()>,
        ),
    )(input)
    .map_err(|_| Error::other("Invalid input"))?;

    Ok(result.iter().map(|(x, y)| IVec2::new(*x, *y)).collect())
}

#[tracing::instrument]
pub fn process(_input: &str, size: usize, num_bytes: usize) -> miette::Result<String, AocError> {
    let byte_coordinates = parse(_input)?;

    let bytes_taken = byte_coordinates
        .iter()
        .copied()
        .take(num_bytes)
        .collect::<Vec<_>>();

    let shortest_path = dijkstra(
        &IVec2::new(0, 0),
        |coords| {
            DIRECTIONS
                .iter()
                .filter_map(|dir| {
                    let successor = coords + IVec2::from(*dir);
                    if !bytes_taken.contains(&successor)
                        && (0..size as i32).contains(&coords.x)
                        && (0..size as i32).contains(&coords.y)
                    {
                        Some((successor, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(IVec2, usize)>>()
        },
        |IVec2 { x, y }| *x == size as i32 - 1 && *y == size as i32 - 1,
    );

    Ok(shortest_path
        .ok_or(Error::other("No path found"))?
        .1
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("22", process(input, 7, 12)?);
        Ok(())
    }
}
