use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated, tuple};
use std::io::Error;
use std::ops::{BitXor, Shr};

struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

struct Program {
    initial_registers: Registers,
    instructions: Vec<u8>,
}

fn parse(input: &str) -> Result<Program, AocError> {
    let (_, parsed) = tuple::<_, _, (), _>((
        terminated(preceded(tag("Register A: "), complete::u32), tag("\n")),
        terminated(preceded(tag("Register B: "), complete::u32), tag("\n")),
        terminated(preceded(tag("Register C: "), complete::u32), tag("\n\n")),
        preceded(tag("Program: "), separated_list0(tag(","), complete::u8)),
    ))(input)
    .map_err(|_| Error::other("Invalid input"))?;

    Ok(Program {
        initial_registers: Registers {
            a: parsed.0,
            b: parsed.1,
            c: parsed.2,
        },
        instructions: parsed.3,
    })
}

// Direct implementation of the program
fn program(mut a: u64) -> Vec<u8> {
    let mut output = Vec::new();

    while a != 0 {
        let b_1 = a % 8;
        let b_2 = b_1.bitxor(7);
        let c = a.shr(b_2);
        let b_3 = b_2.bitxor(7);
        let b_4 = b_3.bitxor(c);
        a = a.shr(3);
        output.push((b_4 % 8) as u8);
    }

    output
}

fn find_solution(a: u64, original_instructions: &Vec<u8>) -> Option<u64> {
    let output = program(a);
    let output_reversed: Vec<u8> = output.iter().copied().rev().collect();
    let original_reversed_partial: Vec<u8> = original_instructions
        .iter()
        .copied()
        .rev()
        .take(output.len())
        .collect();

    if output == original_instructions.clone() {
        Some(a)
    } else if output_reversed != original_reversed_partial {
        None
    } else {
        println!("{} -> {:?}", a, &output);
        if a == 0 { 1..8 } else { 0..8 }
            .into_iter()
            .filter_map(|i| find_solution(a * 8 + i as u64, original_instructions))
            .next()
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let original_program = parse(_input)?;

    let solution = find_solution(0, &original_program.instructions);

    Ok(solution.unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() -> miette::Result<()> {
        let input = 66245665_u64;
        assert_eq!(vec![1, 4, 6, 1, 6, 4, 3, 0, 3], program(input));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 1,4,6,1,6,4,3,0,3";
        assert_eq!("66245665", process(input)?);
        Ok(())
    }
}
