advent_of_code::solution!(2);

type Output = u64;

fn parse_input(input: &str) -> Vec<Vec<Output>> {
    let line_parser = nom::multi::separated_list1(
        nom::character::complete::space1::<&str, ()>,
        nom::character::complete::u64,
    );

    let mut parser =
        nom::multi::separated_list1(nom::character::complete::line_ending, line_parser);

    parser(input).unwrap().1
}

fn is_safe(levels: &[Output]) -> bool {
    let safe_differences = levels
        .windows(2)
        .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));
    let increasing = levels.windows(2).all(|w| w[0] < w[1]);
    let decreasing = levels.windows(2).all(|w| w[0] > w[1]);

    safe_differences && (increasing || decreasing)
}

fn is_safe_dampened(levels: &[Output]) -> bool {
    if is_safe(levels) {
        return true;
    }

    let mut dropper = Vec::new();

    for i in 0..levels.len() {
        let l = &levels[..i];
        let r = &levels[i + 1..];
        dropper.clear();
        dropper.extend_from_slice(l);
        dropper.extend_from_slice(r);
        if is_safe(&dropper) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<Output> {
    let report = parse_input(input);
    Some(report.into_iter().filter(|x| is_safe(x)).count() as Output)
}

pub fn part_two(input: &str) -> Option<Output> {
    let report = parse_input(input);
    Some(report.into_iter().filter(|x| is_safe_dampened(x)).count() as Output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
