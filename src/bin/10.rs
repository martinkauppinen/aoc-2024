use std::{
    collections::{HashSet, VecDeque},
    ops::{Index, IndexMut},
};

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            neighbors.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        neighbors.push(Point {
            x: self.x + 1,
            y: self.y,
        });
        if self.y > 0 {
            neighbors.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        neighbors.push(Point {
            x: self.x,
            y: self.y + 1,
        });
        neighbors
    }

    fn in_bounds(&self, width: usize, height: usize) -> bool {
        (0..width).contains(&self.x) && (0..height).contains(&self.y)
    }
}

struct Grid(Vec<Vec<u8>>);

impl Index<Point> for Grid {
    type Output = u8;
    fn index(&self, point: Point) -> &Self::Output {
        &self.0[point.y][point.x]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.0[point.y][point.x]
    }
}

fn parse_input(input: &str) -> (Grid, Vec<Point>) {
    let mut grid = Vec::new();
    let mut zeroes = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let num = c.to_digit(10).unwrap() as u8;
            if num == 0 {
                zeroes.push(Point { x, y });
            }
            row.push(num);
        }
        grid.push(row);
    }
    (Grid(grid), zeroes)
}

fn get_next_higher(grid: &Grid, point: Point) -> Vec<Point> {
    let mut step_ups = Vec::new();
    for neighbor in point
        .neighbors()
        .into_iter()
        .filter(|n| n.in_bounds(grid.0[0].len(), grid.0.len()))
    {
        if grid[neighbor] == grid[point] + 1 {
            step_ups.push(neighbor);
        }
    }
    step_ups
}

fn dfs(grid: &Grid, reached_nines: &mut HashSet<Point>, point: Point) {
    if grid[point] == 9 {
        reached_nines.insert(point);
        return;
    }

    let step_ups = get_next_higher(grid, point);
    for step_up in step_ups {
        dfs(grid, reached_nines, step_up);
    }
}

fn score_zero(grid: &Grid, zero: Point) -> usize {
    let mut reached_nines = HashSet::new();
    dfs(grid, &mut reached_nines, zero);
    reached_nines.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, zeroes) = parse_input(input);
    let mut count = 0;

    for zero in zeroes {
        count += score_zero(&grid, zero);
    }

    Some(count)
}

fn bfs(grid: &Grid, zero: Point) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(zero);
    let mut num_paths = 0;

    while let Some(point) = queue.pop_front() {
        if grid[point] == 9 {
            num_paths += 1;
            continue;
        }
        for neighbor in get_next_higher(grid, point) {
            queue.push_back(neighbor);
        }
    }

    num_paths
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, zeroes) = parse_input(input);
    let mut count = 0;

    for zero in zeroes {
        count += bfs(&grid, zero);
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_example_two() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let (grid, zeroes) = parse_input(&input);
        let expected_scores = vec![5, 6, 5, 3, 1, 3, 5, 3, 5];
        for (zero, expected_score) in zeroes.iter().zip(expected_scores) {
            let score = score_zero(&grid, *zero);
            assert_eq!(score, expected_score);
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two_individually() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let (grid, zeroes) = parse_input(&input);
        let expected_ratings = vec![20, 24, 10, 4, 1, 4, 5, 8, 5];
        for (zero, expected_score) in zeroes.iter().zip(expected_ratings) {
            let score = bfs(&grid, *zero);
            assert_eq!(score, expected_score);
        }
    }
}
