use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

const STEPS: usize = 100;

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

#[tracing::instrument]
pub fn process(_input: &str, height: u32, width: u32) -> miette::Result<String, AocError> {
    let (_, robots) = parse(_input).unwrap();

    let safety_factor = robots
        .iter()
        .filter_map(|robot| {
            let final_position_unwrapped = robot.position + robot.velocity * STEPS as i32;
            let final_position_x = final_position_unwrapped.x.rem_euclid(width as i32) as u32;
            let final_position_y = final_position_unwrapped.y.rem_euclid(height as i32) as u32;

            let (middle_x, middle_y) = (width / 2, height / 2);

            let quadrant = if (0..middle_x).contains(&final_position_x)
                && (0..middle_y).contains(&final_position_y)
            {
                Some(1)
            } else if (0..middle_x).contains(&final_position_x)
                && (middle_y + 1..height).contains(&final_position_y)
            {
                Some(2)
            } else if (middle_x + 1..width).contains(&final_position_x)
                && (middle_y + 1..height).contains(&final_position_y)
            {
                Some(3)
            } else if (middle_x + 1..width).contains(&final_position_x)
                && (0..middle_y).contains(&final_position_y)
            {
                Some(4)
            } else {
                None
            };

            quadrant
        })
        .counts()
        .values()
        .product::<usize>();

    Ok(safety_factor.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input, 7, 11)?);
        Ok(())
    }
}
