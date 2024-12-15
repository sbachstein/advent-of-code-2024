use crate::custom_error::AocError;
use glam::IVec2;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Debug)]
enum LocationContent {
    Empty,
    Obstacle,
}

#[derive(Debug)]
struct GuardMap {
    map: HashMap<IVec2, LocationContent>,
    guard_position: IVec2,
    guard_direction: IVec2,
}

fn parse(input: &str) -> GuardMap {
    let mut map = HashMap::new();
    let mut guard_position: IVec2 = IVec2::default();
    let mut guard_direction: IVec2 = IVec2::default();

    for (y, line) in input.lines().enumerate() {
        for (x, pos) in line.chars().enumerate() {
            let entry = match pos {
                '.' => (IVec2::new(x as i32, y as i32), LocationContent::Empty),
                '#' => (IVec2::new(x as i32, y as i32), LocationContent::Obstacle),
                '^' => {
                    guard_position = IVec2::new(x as i32, y as i32);
                    guard_direction = IVec2::new(0, -1);
                    (IVec2::new(x as i32, y as i32), LocationContent::Empty)
                }
                '>' => {
                    guard_position = IVec2::new(x as i32, y as i32);
                    guard_direction = IVec2::new(1, 0);
                    (IVec2::new(x as i32, y as i32), LocationContent::Empty)
                }
                'v' => {
                    guard_position = IVec2::new(x as i32, y as i32);
                    guard_direction = IVec2::new(0, 1);
                    (IVec2::new(x as i32, y as i32), LocationContent::Empty)
                }
                '<' => {
                    guard_position = IVec2::new(x as i32, y as i32);
                    guard_direction = IVec2::new(-1, 0);
                    (IVec2::new(x as i32, y as i32), LocationContent::Empty)
                }
                _ => unreachable!(),
            };

            map.insert(entry.0, entry.1);
        }
    }

    GuardMap {
        map,
        guard_position,
        guard_direction,
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let guard_map = parse(_input);

    let mut visited_places: HashSet<(IVec2, IVec2)> = HashSet::new();
    let mut current_position: IVec2 = guard_map.guard_position.clone();
    let mut current_direction: IVec2 = guard_map.guard_direction.clone();

    while !visited_places.contains(&(current_position, current_direction)) {
        visited_places.insert((current_position, current_direction));
        match guard_map.map.get(&(current_position + current_direction)) {
            Some(LocationContent::Empty) => current_position = current_position + current_direction,
            Some(LocationContent::Obstacle) => {
                current_direction = match current_direction {
                    IVec2 { x: 1, y: 0 } => IVec2::new(0, 1),
                    IVec2 { x: 0, y: 1 } => IVec2::new(-1, 0),
                    IVec2 { x: -1, y: 0 } => IVec2::new(0, -1),
                    IVec2 { x: 0, y: -1 } => IVec2::new(1, 0),
                    _ => unreachable!(),
                }
            }
            None => break,
        }
    }

    let positions = visited_places
        .iter()
        .map(|(pos, _)| *pos)
        .collect::<HashSet<_>>();

    let mut obstacle_count = 0;

    for obstacle_position in positions.iter() {
        let mut adjusted_map = guard_map.map.clone();

        adjusted_map.insert(obstacle_position.clone(), LocationContent::Obstacle);
        visited_places.clear();
        current_position = guard_map.guard_position.clone();
        current_direction = guard_map.guard_direction.clone();

        loop {
            if visited_places.contains(&(current_position, current_direction)) {
                obstacle_count += 1;
                break;
            }
            visited_places.insert((current_position, current_direction));
            match adjusted_map.get(&(current_position + current_direction)) {
                Some(LocationContent::Empty) => {
                    current_position = current_position + current_direction
                }
                Some(LocationContent::Obstacle) => {
                    current_direction = match current_direction {
                        IVec2 { x: 1, y: 0 } => IVec2::new(0, 1),
                        IVec2 { x: 0, y: 1 } => IVec2::new(-1, 0),
                        IVec2 { x: -1, y: 0 } => IVec2::new(0, -1),
                        IVec2 { x: 0, y: -1 } => IVec2::new(1, 0),
                        _ => unreachable!(),
                    }
                }
                None => break,
            }
        }
    }

    Ok(obstacle_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
