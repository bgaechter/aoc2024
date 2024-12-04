use std::fs;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Multiply(u32, u32),
    Do,
    Dont,
}

fn parse_input(input: &str) -> u32 {
    let instructions = extract_instructions(input);
    process_instructions(&instructions)
}

fn extract_instructions(input: &str) -> Vec<(usize, Instruction)> {
    let mul_instructions = extract_multiplications(input);
    let do_instructions = extract_dos(input);
    let dont_instructions = extract_donts(input);

    mul_instructions
        .into_iter()
        .chain(do_instructions)
        .chain(dont_instructions)
        .collect::<Vec<_>>()
        .into_iter()
        .sorted_by_key(|(pos, _)| *pos)
        .collect()
}

fn extract_multiplications(input: &str) -> Vec<(usize, Instruction)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .filter_map(|cap| {
            let pos = cap.get(0)?.start();
            let x = cap[1].parse::<u32>().ok()?;
            let y = cap[2].parse::<u32>().ok()?;
            Some((pos, Instruction::Multiply(x, y)))
        })
        .collect()
}

fn extract_dos(input: &str) -> Vec<(usize, Instruction)> {
    Regex::new(r"do\(\)")
        .unwrap()
        .find_iter(input)
        .map(|m| (m.start(), Instruction::Do))
        .collect()
}

fn extract_donts(input: &str) -> Vec<(usize, Instruction)> {
    Regex::new(r"don't\(\)")
        .unwrap()
        .find_iter(input)
        .map(|m| (m.start(), Instruction::Dont))
        .collect()
}

fn process_instructions(instructions: &[(usize, Instruction)]) -> u32 {
    let (sum, _) = instructions
        .iter()
        .fold((0, true), |(sum, enabled), (_, instruction)| {
            match instruction {
                Instruction::Multiply(x, y) if enabled => (sum + x * y, enabled),
                Instruction::Do => (sum, true),
                Instruction::Dont => (sum, false),
                _ => (sum, enabled)
            }
        });
    sum
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let result = parse_input(&input);
    println!("Sum of all enabled multiplications: {}", result);
    Ok(())
}