use itertools::Itertools;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let line_parser = nom::sequence::separated_pair(
        nom::character::complete::u64::<&str, ()>,
        nom::character::complete::space1,
        nom::character::complete::u64,
    );
    let mut parser =
        nom::multi::separated_list1(nom::character::complete::line_ending, line_parser);
    let (_, pairs) = parser(input).unwrap();
    let (mut a, mut b): (Vec<u64>, Vec<u64>) = pairs.into_iter().multiunzip();

    a.sort_unstable();
    b.sort_unstable();

    (a, b)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (a, b) = parse_input(input);
    Some(a.into_iter().zip(b).map(|(x, y)| x.abs_diff(y)).sum())
}

// Naive initial solution. Looks short, but is comparatively slow.
pub fn part_two_naive(input: &str) -> Option<u32> {
    let (a, b) = parse_input(input);
    Some(a.into_iter().fold(0, |acc, x| {
        acc + b
            .iter()
            .skip_while(|y| x != **y)
            .take_while(|y| x == **y)
            .count() as u32
            * x as u32
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (a, b) = parse_input(input);
    let a = a.chunk_by(std::cmp::PartialEq::eq);
    let mut b = b.chunk_by(std::cmp::PartialEq::eq).peekable();

    let mut similarity: u64 = 0;
    for chunk in a {
        let elem = chunk[0];

        while b.peek().is_some_and(|c| c[0] < elem) {
            b.next();
        }

        if b.peek().is_some_and(|c| c[0] == elem) {
            let other = b.next().unwrap();
            let n = other.len() as u64;
            let m = chunk.len() as u64;
            similarity += elem * n * m;
        }
    }

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_naive(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
