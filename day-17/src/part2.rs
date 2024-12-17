use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated, tuple};
use std::io::Error;
use std::ops::{BitXor, Shr};

#[derive(Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

struct Program {
    initial_registers: Registers,
    instructions: Vec<u8>,
}

fn parse(input: &str) -> Result<Program, AocError> {
    let (_, parsed) = tuple::<_, _, (), _>((
        terminated(preceded(tag("Register A: "), complete::u64), tag("\n")),
        terminated(preceded(tag("Register B: "), complete::u64), tag("\n")),
        terminated(preceded(tag("Register C: "), complete::u64), tag("\n\n")),
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

fn retrieve_combo_operand_value(operator: &u8, registers: &Registers) -> Result<u64, AocError> {
    match operator {
        i @ (0..=3) => Ok(*i as u64),
        4 => Ok(registers.a),
        5 => Ok(registers.b),
        6 => Ok(registers.c),
        _ => Err(AocError::IoError(Error::other("Bad combo operator"))),
    }
}

fn execute_program(program: &Program, a: u64) -> Result<Vec<u8>, AocError> {
    let mut registers = program.initial_registers.clone();
    let mut instruction_pointer: usize = 0;
    let mut output_buffer: Vec<u8> = Vec::new();

    registers.a = a;

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
                registers.b = registers.b.bitxor(*operand as u64);
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
                output_buffer.push((retrieve_combo_operand_value(operand, &registers)? % 8) as u8);
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

    Ok(output_buffer)
}

fn find_solution(a: u64, program: &Program) -> Option<u64> {
    let output = execute_program(&program, a).ok()?;
    let output_reversed: Vec<u8> = output.iter().copied().rev().collect();
    let original_reversed_partial: Vec<u8> = program
        .instructions
        .iter()
        .copied()
        .rev()
        .take(output.len())
        .collect();

    if output == program.instructions.clone() {
        Some(a)
    } else if output_reversed != original_reversed_partial {
        None
    } else {
        println!("{} -> {:?}", a, &output);
        (0..8)
            .into_iter()
            .filter_map(|i| find_solution(a * 8 + i as u64, program))
            .next()
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let program = parse(_input)?;

    let solution = (1..8)
        .into_iter()
        .filter_map(|i| find_solution(i, &program))
        .next();

    Ok(solution.unwrap().to_string())
}
