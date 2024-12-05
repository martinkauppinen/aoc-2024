use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::char, multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(5);

// Assumption: page numbers are at most 99
type PageOrderingRules = [Vec<u32>; 100];
type Update = Vec<u32>;

#[derive(PartialEq, Eq)]
struct Page<'a>(u32, &'a PageOrderingRules);

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.1[self.0 as usize].contains(&other.0) {
            std::cmp::Ordering::Less
        } else if other.1[other.0 as usize].contains(&self.0) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn parse_input(input: &str) -> (PageOrderingRules, Vec<Update>) {
    let rule_parser = separated_pair(
        nom::character::complete::u32::<&str, ()>,
        char('|'),
        nom::character::complete::u32,
    );
    let report_parser = separated_list1(tag(","), nom::character::complete::u32::<&str, ()>);

    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut page_ordering_rules: PageOrderingRules = [const { Vec::new() }; 100];
    rules
        .lines()
        .flat_map(rule_parser)
        .for_each(|(_, (before, after))| {
            page_ordering_rules[before as usize].push(after);
        });

    let updates = updates
        .lines()
        .flat_map(report_parser)
        .map(|(_, v)| v)
        .collect();

    (page_ordering_rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    Some(
        updates
            .into_iter()
            .map(|update| update.into_iter().map(|p| Page(p, &rules)).collect_vec())
            .filter(|update| update.is_sorted())
            .map(|update| update[update.len() / 2].0)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    Some(
        updates
            .into_iter()
            .map(|update| update.into_iter().map(|p| Page(p, &rules)).collect_vec())
            .filter(|update| !update.is_sorted())
            .map(|mut update| {
                update.sort();
                update[update.len() / 2].0
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
