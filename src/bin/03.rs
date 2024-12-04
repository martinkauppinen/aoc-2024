use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
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
    fn new(instructions: VecDeque<Instruction>) -> Self {
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
            Some(Instruction::Mul(a, b)) => {
                if self.mul_enabled {
                    return Some(a * b);
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
    Mul(Output, Output),
}

fn instruction_parser(input: &str) -> nom::IResult<&str, Instruction> {
    let do_parser = map(tag("do()"), |_| Instruction::Do);
    let dont_parser = map(tag("don't()"), |_| Instruction::Dont);
    let mul_parser = map(
        preceded(
            tag("mul"),
            delimited(
                char('('),
                separated_pair(
                    nom::character::complete::u32,
                    char(','),
                    nom::character::complete::u32,
                ),
                char(')'),
            ),
        ),
        |(a, b)| Instruction::Mul(a, b),
    );
    alt((do_parser, mul_parser, dont_parser))(input)
}

fn parse_input(input: &str) -> VecDeque<Instruction> {
    let mut preceded_mul_parser = many0(map(many_till(anychar, instruction_parser), |(_, x)| x));

    preceded_mul_parser(input).unwrap().1.into()
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    Some(
        instructions
            .into_iter()
            .filter_map(|x| match x {
                Instruction::Mul(a, b) => Some((a, b)),
                _ => None,
            })
            .map(|(a, b)| a * b)
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
