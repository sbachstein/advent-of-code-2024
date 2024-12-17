use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated, tuple};
use std::io::Error;
use std::ops::{BitXor, Shr};

#[derive(Clone)]
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

fn retrieve_combo_operand_value(operator: &u8, registers: &Registers) -> Result<u32, AocError> {
    match operator {
        i @ (0..=3) => Ok(*i as u32),
        4 => Ok(registers.a),
        5 => Ok(registers.b),
        6 => Ok(registers.c),
        _ => Err(AocError::IoError(Error::other("Bad combo operator"))),
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let program = parse(_input)?;

    let mut registers = program.initial_registers.clone();
    let mut instruction_pointer: usize = 0;
    let mut output_buffer: Vec<u32> = Vec::new();

    while let Some(instruction) = program.instructions.get(instruction_pointer) {
        let operand = program
            .instructions
            .get(instruction_pointer + 1)
            .ok_or(Error::other("Operand missing"))?;
        match instruction {
            0 => {
                // adv
                let numerator = registers.a;
                let num_shifts = retrieve_combo_operand_value(operand, &registers)?;

                registers.a = numerator.shr(num_shifts);
            }
            1 => {
                // bxl
                registers.b = registers.b.bitxor(*operand as u32);
            }
            2 => {
                // bst
                registers.b = retrieve_combo_operand_value(operand, &registers)? % 8;
            }
            3 => {
                // jnz
                if registers.a != 0 {
                    instruction_pointer = *operand as usize;
                    continue;
                }
            }
            4 => {
                // bxc
                registers.b = registers.b.bitxor(registers.c);
            }
            5 => {
                // out
                output_buffer.push(retrieve_combo_operand_value(operand, &registers)? % 8);
            }
            6 => {
                // bdv
                let numerator = registers.a;
                let num_shifts = retrieve_combo_operand_value(operand, &registers)?;

                registers.b = numerator.shr(num_shifts);
            }
            7 => {
                // cdv
                let numerator = registers.a;
                let num_shifts = retrieve_combo_operand_value(operand, &registers)?;

                registers.c = numerator.shr(num_shifts);
            }
            _ => {} // Why?
        }
        instruction_pointer += 2;
    }

    Ok(output_buffer
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }
}
