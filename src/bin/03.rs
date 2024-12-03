use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::{many0, many_till},
    sequence::{delimited, preceded, separated_pair},
};

advent_of_code::solution!(3);

type Output = u32;

struct Machine {
    instructions: VecDeque<Instruction>,
    mul_enabled: bool,
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        let instructions = VecDeque::from(instructions);
        Self {
            instructions,
            mul_enabled: true,
        }
    }

    fn step(&mut self) -> Option<Output> {
        match self.instructions.pop_front() {
            Some(Instruction::Do) => {
                self.mul_enabled = true;
            }
            Some(Instruction::Dont) => {
                self.mul_enabled = false;
            }
            Some(Instruction::Mul(m)) => {
                if self.mul_enabled {
                    return Some(m.execute());
                }
            }
            _ => (),
        }
        None
    }

    fn execute(mut self) -> Output {
        let mut result = 0;
        while !self.instructions.is_empty() {
            result += self.step().unwrap_or(0);
        }
        result
    }
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(Mul),
}

#[derive(Debug)]
struct Mul(Output, Output);

fn instruction_parser(input: &[u8]) -> nom::IResult<&[u8], Instruction> {
    let do_parser = map(tag("do()"), |_| Instruction::Do);
    let dont_parser = map(tag("don't()"), |_| Instruction::Dont);
    let mul_parser = map(
        preceded(
            tag(b"mul"),
            delimited(
                tag(b"("),
                separated_pair(
                    nom::character::complete::u32,
                    tag(b","),
                    nom::character::complete::u32,
                ),
                tag(b")"),
            ),
        ),
        |(a, b)| Instruction::Mul(Mul(a, b)),
    );
    alt((do_parser, mul_parser, dont_parser))(input)
}

impl Mul {
    pub fn execute(self) -> Output {
        self.0 * self.1
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut preceded_mul_parser = many0(map(many_till(anychar, instruction_parser), |(_, x)| x));

    preceded_mul_parser(input.as_bytes()).unwrap().1
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    Some(
        instructions
            .into_iter()
            .filter_map(|x| match x {
                Instruction::Mul(x) => Some(x),
                _ => None,
            })
            .map(Mul::execute)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let machine = Machine::new(instructions);
    Some(machine.execute())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(161085926));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(82045421));
    }
}
