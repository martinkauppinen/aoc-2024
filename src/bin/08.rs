use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn in_bounds(&self, width: i64, height: i64) -> bool {
        (0..width).contains(&self.x) && (0..height).contains(&self.y)
    }
}

fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, c)| {
                map.entry(c).or_insert_with(Vec::new).push(Point {
                    x: x as i64,
                    y: y as i64,
                });
            });
    });
    map
}

fn bounds(input: &str) -> (i64, i64) {
    let height = input.lines().count() as i64;
    let width = input.lines().next().unwrap().len() as i64;
    (height, width)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut antinodes = Vec::new();
    let (height, width) = bounds(input);

    for (_frequency, positions) in parse_input(input) {
        for p in positions.iter().combinations(2) {
            let diff = *p[1] - *p[0];
            antinodes.push(*p[1] + diff);
            antinodes.push(*p[0] - diff);
        }
    }

    Some(
        antinodes
            .into_iter()
            .filter(|p| p.in_bounds(width, height))
            .unique()
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut antinodes = Vec::new();
    let (height, width) = bounds(input);

    for (_frequency, positions) in parse_input(input) {
        antinodes.extend_from_slice(&positions);
        for p in positions.iter().combinations(2) {
            let diff = *p[1] - *p[0];

            let mut node_1 = *p[1] + diff;
            let mut node_2 = *p[0] - diff;

            while node_1.in_bounds(width, height) {
                antinodes.push(node_1);
                node_1 = node_1 + diff;
            }

            while node_2.in_bounds(width, height) {
                antinodes.push(node_2);
                node_2 = node_2 - diff;
            }
        }
    }

    Some(
        antinodes
            .into_iter()
            .filter(|p| p.in_bounds(width, height))
            .unique()
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
