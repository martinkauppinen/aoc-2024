use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        let mut dp = [const { Vec::new() }; 2];
        dp[0].resize(self.numbers.len(), Vec::<u64>::new()); // add
        dp[1].resize(self.numbers.len(), Vec::<u64>::new()); // mul

        for (i, number) in self.numbers.iter().enumerate().rev() {
            let next = if i == self.numbers.len() - 1 {
                vec![self.target]
            } else {
                let mut next = dp[0][i + 1].clone();
                next.extend_from_slice(&dp[1][i + 1]);
                next
            };

            dp[0][i] = next.iter().filter_map(|n| n.checked_sub(*number)).collect();
            dp[1][i] = next
                .iter()
                .filter(|n| *n % number == 0)
                .map(|n| n / *number)
                .collect();
        }
        dp[0][0].contains(&0) || dp[1][0].contains(&1)
    }

    fn is_valid_with_concatenation(&self) -> bool {
        let mut dp = [const { Vec::new() }; 3];
        dp[0].resize(self.numbers.len(), Vec::<u64>::new()); // add
        dp[1].resize(self.numbers.len(), Vec::<u64>::new()); // mul
        dp[2].resize(self.numbers.len(), Vec::<u64>::new()); // cat

        for (i, number) in self.numbers.iter().enumerate().rev() {
            let next = if i == self.numbers.len() - 1 {
                vec![self.target]
            } else {
                let mut next = dp[0][i + 1].clone();
                next.extend_from_slice(&dp[1][i + 1]);
                next.extend_from_slice(&dp[2][i + 1]);
                next
            };

            dp[0][i] = next.iter().filter_map(|n| n.checked_sub(*number)).collect();
            dp[1][i] = next
                .iter()
                .filter(|n| *n % number == 0)
                .map(|n| n / *number)
                .collect();
            dp[2][i] = next
                .iter()
                .filter(|n| {
                    let num_digits = number.ilog10() + 1;
                    let modulus = 10u64.pow(num_digits);
                    *n % modulus == *number
                })
                .map(|n| n / 10u64.pow(number.ilog10() + 1))
                .collect();
        }

        dp[0][0].contains(&0) || dp[1][0].contains(&1) || dp[2][1].contains(&self.numbers[0])
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    let target_parser = nom::character::complete::u64::<&str, ()>;
    let numbers_parser = separated_list1(tag(" "), nom::character::complete::u64);
    let line_parser = nom::sequence::separated_pair(target_parser, tag(": "), numbers_parser);
    input
        .lines()
        .flat_map(line_parser)
        .map(|(_, (target, numbers))| Equation { target, numbers })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(
        equations
            .into_iter()
            .filter(Equation::is_valid)
            .map(|equation| equation.target)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(
        equations
            .into_iter()
            .filter(Equation::is_valid_with_concatenation)
            .map(|equation| equation.target)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
