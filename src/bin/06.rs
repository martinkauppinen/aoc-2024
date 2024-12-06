advent_of_code::solution!(6);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

impl Point {
    fn checked_add(self, direction: Direction, bottom_right: Point) -> Option<Point> {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    Some(Point {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < bottom_right.y {
                    Some(Point {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(Point {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < bottom_right.x {
                    Some(Point {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Visited(u8),
    Obstruction,
    InsertedObstruction,
}

#[derive(Clone)]
struct Guard {
    facing: Direction,
    position: Point,
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    guard: Guard,
    starting_position: Point,
    grid: [[Tile; 130]; 130],
}

impl Map {
    fn visit(&mut self, point: Point) {
        match self.grid[point.y][point.x] {
            Tile::Empty => {
                self.grid[point.y][point.x] = Tile::Visited(self.guard.facing as u8);
            }
            Tile::Visited(ref mut direction_mask) => {
                *direction_mask |= self.guard.facing as u8;
            }
            _ => (),
        }
    }

    // returns new position and whether it has been visited before, in the same direction
    fn patrol(&mut self) -> Option<(Point, bool)> {
        let next_pos = self.guard.position.checked_add(
            self.guard.facing,
            Point {
                x: self.width - 1,
                y: self.height - 1,
            },
        )?;

        match self.grid[next_pos.y][next_pos.x] {
            Tile::Visited(direction_mask) => {
                self.guard.position = next_pos;
                self.visit(next_pos);

                let already_visited = self.guard.facing as u8 & direction_mask != 0;

                Some((next_pos, already_visited))
            }
            Tile::Empty => {
                self.guard.position = next_pos;
                self.visit(next_pos);
                Some((next_pos, false))
            }
            _ => {
                self.guard.facing = self.guard.facing.turn();
                self.visit(self.guard.position);
                Some((self.guard.position, false))
            }
        }
    }

    // returns true if loops
    fn loops(mut self) -> bool {
        while let Some((_, visited_before)) = self.patrol() {
            if visited_before {
                return true;
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = Map {
        width: 0,
        height: 0,
        guard: Guard {
            facing: Direction::Up,
            position: Point { x: 0, y: 0 },
        },
        starting_position: Point { x: 0, y: 0 },
        grid: [[Tile::Empty; 130]; 130],
    };
    map.width = input.lines().next().unwrap().len();
    map.height = input.lines().count();

    input.lines().enumerate().for_each(|(y, l)| {
        l.bytes().enumerate().for_each(|(x, c)| {
            if c == b'#' {
                map.grid[y][x] = Tile::Obstruction;
            } else if c == b'^' {
                map.guard.position = Point { x, y };
                map.starting_position = Point { x, y };
                map.visit(map.guard.position);
            }
        })
    });

    map
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse_input(input);
    while map.patrol().is_some() {}
    Some(
        map.grid
            .into_iter()
            .flatten()
            .filter(|&t| matches!(t, Tile::Visited(..)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut finished_map = map.clone();

    // cache, no threading ---------------------------------------
    let mut obstruction_count = 0;
    loop {
        let mut edited_map = finished_map.clone();
        let p = edited_map.guard.position.checked_add(
            edited_map.guard.facing,
            Point {
                x: edited_map.width - 1,
                y: edited_map.height - 1,
            },
        );

        if p.is_some_and(|p| edited_map.grid[p.y][p.x] == Tile::Empty) {
            let p = p.unwrap();
            edited_map.grid[p.y][p.x] = Tile::InsertedObstruction;
            if edited_map.loops() {
                obstruction_count += 1;
            }
        }

        if finished_map.patrol().is_none() {
            break;
        }
    }
    Some(obstruction_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
