use crate::custom_error::AocError;
use nalgebra::{Matrix2, Vector2};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};
use nom::Parser;

#[derive(Debug)]
struct Machine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    target: (u64, u64),
}

fn parse(input: &str) -> Vec<Machine> {
    let (_input, parsed_machines) = separated_list0(
        tag::<&str, &str, ()>("\n\n"),
        tuple((
            preceded(tag("Button A: X+"), complete::u64),
            preceded(tag(", Y+"), complete::u64),
            tag("\n"),
            preceded(tag("Button B: X+"), complete::u64),
            preceded(tag(", Y+"), complete::u64),
            tag("\n"),
            preceded(tag("Prize: X="), complete::u64),
            preceded(tag(", Y="), complete::u64),
        )),
    )(input)
    .unwrap();

    parsed_machines
        .iter()
        .map(|parsed| Machine {
            button_a: (parsed.0, parsed.1),
            button_b: (parsed.3, parsed.4),
            target: (parsed.6, parsed.7),
        })
        .collect()
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let machines = parse(_input);

    let result = machines
        .iter()
        .map(|machine| {
            let A = Matrix2::new(
                machine.button_a.0 as f64,
                machine.button_b.0 as f64,
                machine.button_a.1 as f64,
                machine.button_b.1 as f64,
            );

            let target = Vector2::new(machine.target.0 as f64, machine.target.1 as f64);

            let maybe_solution = A.lu().solve(&target);

            dbg!(&maybe_solution);

            match maybe_solution {
                Some(solution) => {
                    let solution_a = solution.x.round() as u64;
                    let solution_b = solution.y.round() as u64;
                    if solution_a * machine.button_a.0 + solution_b * machine.button_b.0
                        == machine.target.0
                        && solution_a * machine.button_a.1 + solution_b * machine.button_b.1
                            == machine.target.1
                    {
                        3 * solution_a + solution_b
                    } else {
                        0
                    }
                }
                None => 0,
            }
        })
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
