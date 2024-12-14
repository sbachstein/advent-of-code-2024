use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

const STEPS: usize = 100000;

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    let result = separated_list0(
        line_ending,
        separated_pair(
            preceded(
                tag("p="),
                separated_pair(complete::i32, tag(","), complete::i32),
            ),
            tag(" "),
            preceded(
                tag("v="),
                separated_pair(complete::i32, tag(","), complete::i32),
            ),
        ),
    )(input)
    .map(|(input, output)| {
        let robots = output
            .into_iter()
            .map(|(position, velocity)| Robot {
                position: IVec2::new(position.0, position.1),
                velocity: IVec2::new(velocity.0, velocity.1),
            })
            .collect();

        (input, robots)
    });

    result
}

fn visualize_grid(grid: &HashMap<IVec2, char>, height: u32, width: u32) -> String {
    (0..height)
        .map(|y| {
            (0..width)
                .map(move |x| grid[&IVec2::new(x as i32, y as i32)].to_string())
                .join("")
        })
        .join("\n")
}

#[tracing::instrument]
pub fn process(_input: &str, height: u32, width: u32) -> miette::Result<String, AocError> {
    let (_, mut robots) = parse(_input).unwrap();

    for step in 1..=STEPS {
        let mut grid = (0..width)
            .cartesian_product(0..height)
            .map(|(x, y)| (IVec2::new(x as i32, y as i32), '.'))
            .collect::<HashMap<IVec2, char>>();

        for robot in robots.iter_mut() {
            let new_position_unwrapped = robot.position + robot.velocity;
            robot.position = IVec2::new(
                new_position_unwrapped.x.rem_euclid(width as i32),
                new_position_unwrapped.y.rem_euclid(height as i32),
            );
            grid.insert(robot.position, '#');
        }

        let grid_visualization = visualize_grid(&grid, height, width);
        if grid_visualization.contains("##########") {
            println!("{}", grid_visualization);
            println!("After {} steps", step);
            return Ok(step.to_string());
        }
    }

    Ok("0".to_string())
}
