use std::collections::HashMap;

use nom::{bytes::complete::tag, multi::separated_list0};

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u128> {
    separated_list0(tag(" "), nom::character::complete::u128::<&str, ()>)(input)
        .unwrap()
        .1
}

fn solve_iterations(stones: Vec<u128>, iterations: usize) -> Option<u128> {
    let mut number_count = HashMap::<u128, u128>::new();

    stones.into_iter().for_each(|stone| {
        number_count
            .entry(stone)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    for _ in 0..iterations {
        let mut copy = number_count.clone();
        for (number, count) in number_count.iter() {
            let num_digits = number.checked_ilog10().unwrap_or(0) + 1;
            if *number == 0 {
                *copy.entry(1).or_insert(0) += *count;
            } else if num_digits % 2 == 0 {
                let modulus = 10u128.pow((num_digits + 1) / 2);
                let left = number / modulus;
                let right = number % modulus;

                *copy.entry(left).or_insert(0) += *count;
                *copy.entry(right).or_insert(0) += *count;
            } else {
                *copy.entry(*number * 2024).or_insert(0) += *count;
            }
            copy.entry(*number).and_modify(|c| *c -= *count);
        }
        number_count = copy;
    }
    Some(number_count.values().sum())
}

pub fn part_one(input: &str) -> Option<u128> {
    let stones = parse_input(input);
    solve_iterations(stones, 25)
}

pub fn part_two(input: &str) -> Option<u128> {
    let stones = parse_input(input);
    solve_iterations(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
