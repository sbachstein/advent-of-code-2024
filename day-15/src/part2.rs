use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Error;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Object {
    Robot,
    BoxLeftside,
    BoxRightside,
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
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let coordinates = IVec2::new(x as i32, y as i32);
                match c {
                    '[' => Some((coordinates, Object::BoxLeftside)),
                    ']' => Some((coordinates, Object::BoxRightside)),
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

fn can_move_object(
    grid: &mut HashMap<IVec2, Object>,
    coordinates: &IVec2,
    direction: &Direction,
) -> bool {
    let new_coordinates = coordinates + get_coordinates_from_direction(direction);

    match grid.get(coordinates) {
        Some(side @ (Object::BoxLeftside | Object::BoxRightside)) => {
            let new_coordinates_other_side = coordinates
                + get_coordinates_from_direction(match side {
                    Object::BoxLeftside => &Direction::Right,
                    Object::BoxRightside => &Direction::Left,
                    _ => unreachable!(),
                })
                + get_coordinates_from_direction(direction);

            match direction {
                Direction::Up | Direction::Down => {
                    can_move_object(grid, &new_coordinates, direction)
                        && can_move_object(grid, &(new_coordinates_other_side), direction)
                }
                Direction::Left | Direction::Right => {
                    can_move_object(grid, &new_coordinates_other_side, direction)
                }
            }
        }
        Some(Object::Robot) => can_move_object(grid, &new_coordinates, direction),
        Some(Object::Wall) => false,
        None => true,
    }
}

fn move_object(
    grid: &mut HashMap<IVec2, Object>,
    coordinates: &IVec2,
    direction: &Direction,
) -> bool {
    let new_coordinates = coordinates + get_coordinates_from_direction(direction);

    if can_move_object(grid, &coordinates, direction) {
        match grid.get(coordinates) {
            Some(side @ (Object::BoxLeftside | Object::BoxRightside)) => {
                let coordinates_other_side = coordinates
                    + get_coordinates_from_direction(match side {
                        Object::BoxLeftside => &Direction::Right,
                        Object::BoxRightside => &Direction::Left,
                        _ => unreachable!(),
                    });
                let new_coordinates_other_side =
                    coordinates_other_side + get_coordinates_from_direction(direction);

                match direction {
                    Direction::Up | Direction::Down => {
                        if grid.get(&new_coordinates) == grid.get(&coordinates) {
                            // Box is aligned
                            move_object(grid, &new_coordinates, direction);
                        } else {
                            move_object(grid, &new_coordinates, direction);
                            move_object(grid, &(new_coordinates_other_side), direction);
                        }
                    }
                    Direction::Left | Direction::Right => {
                        move_object(grid, &new_coordinates_other_side, direction);
                    }
                }

                let object = grid.get(&coordinates).unwrap().clone();
                let object_other_side = grid.get(&coordinates_other_side).unwrap().clone();
                grid.remove(&coordinates_other_side);
                grid.insert(new_coordinates, object);
                grid.insert(new_coordinates_other_side, object_other_side);
            }
            Some(Object::Robot) => {
                move_object(grid, &new_coordinates, direction);
                grid.remove(&coordinates);
                grid.insert(new_coordinates, Object::Robot);
            }
            Some(Object::Wall) => {}
            None => {}
        }
        true
    } else {
        false
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
                    Some(Object::BoxLeftside) => '[',
                    Some(Object::BoxRightside) => ']',
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
        //println!(
        //    "{}",
        //    visualize_grid(&grid).unwrap_or("Invalid grid".to_string())
        //);
    }

    let gps_sum = grid
        .iter()
        .filter_map(|(k, v)| {
            if *v == Object::BoxLeftside {
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
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
