use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Error;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Object {
    Robot,
    Box,
    Wall,
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Result<(HashMap<IVec2, Object>, Vec<Direction>), AocError> {
    let (input_grid, input_directions) = input
        .split("\n\n")
        .next_tuple()
        .ok_or(Error::other("Bad input format"))?;

    let grid = input_grid
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let coordinates = IVec2::new(x as i32, y as i32);
                match c {
                    'O' => Some((coordinates, Object::Box)),
                    '#' => Some((coordinates, Object::Wall)),
                    '@' => Some((coordinates, Object::Robot)),
                    _ => None,
                }
            })
        })
        .collect();

    let directions = input_directions
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    Ok((grid, directions))
}

fn get_coordinates_from_direction(direction: &Direction) -> IVec2 {
    match direction {
        Direction::Up => IVec2::new(0, -1),
        Direction::Down => IVec2::new(0, 1),
        Direction::Left => IVec2::new(-1, 0),
        Direction::Right => IVec2::new(1, 0),
    }
}

fn move_object(
    grid: &mut HashMap<IVec2, Object>,
    coordinates: &IVec2,
    direction: &Direction,
) -> bool {
    let object = match grid.get(coordinates) {
        Some(object) => object,
        None => return false,
    }
    .clone();
    let new_coordinates = coordinates + get_coordinates_from_direction(direction);

    match grid.get(&new_coordinates) {
        Some(Object::Box) => {
            if move_object(grid, &new_coordinates, direction) {
                grid.insert(new_coordinates, object);
                grid.remove(coordinates);
                true
            } else {
                false
            }
        }
        None => {
            grid.insert(new_coordinates, object);
            grid.remove(coordinates);
            true
        }
        _ => false,
    }
}

fn visualize_grid(grid: &HashMap<IVec2, Object>) -> Result<String, AocError> {
    let height = grid
        .iter()
        .map(|(k, _)| k.y)
        .max()
        .ok_or(Error::other("Empty grid"))?
        + 1;
    let width = grid
        .iter()
        .map(|(k, _)| k.x)
        .max()
        .ok_or(Error::other("Empty grid"))?
        + 1;

    let string = (0..height as usize)
        .map(|y| {
            (0..width as usize)
                .map(move |x| match grid.get(&IVec2::new(x as i32, y as i32)) {
                    Some(Object::Box) => 'O',
                    Some(Object::Wall) => '#',
                    Some(Object::Robot) => '@',
                    _ => '.',
                })
                .join("")
        })
        .join("\n");

    Ok(string)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (mut grid, directions) = parse(_input)?;
    let mut robot_position = grid
        .iter()
        .find(|(_, v)| **v == Object::Robot)
        .ok_or(Error::other("No robot on grid"))?
        .0
        .clone();

    for direction in directions.iter() {
        if move_object(&mut grid, &robot_position, direction) {
            robot_position += get_coordinates_from_direction(direction);
        }
        //println!("{}", visualize_grid(&grid).unwrap_or("Invalid grid".to_string()));
    }

    let gps_sum = grid
        .iter()
        .filter_map(|(k, v)| {
            if *v == Object::Box {
                Some(100 * k.y + k.x)
            } else {
                None
            }
        })
        .sum::<i32>();

    Ok(gps_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("2028", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", process(input)?);
        Ok(())
    }
}
