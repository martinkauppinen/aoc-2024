use itertools::Itertools;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();

    let mut transposed_grid = vec![vec![0u8; height]; width];
    input.lines().enumerate().for_each(|(i, l)| {
        l.bytes().enumerate().for_each(|(j, c)| {
            transposed_grid[j][i] = c;
        })
    });

    let horizontal = grid.iter().fold(0, |acc, row| {
        acc + row.as_slice().windows(4).fold(0, |acc, window| {
            if window == b"XMAS" || window == b"SAMX" {
                acc + 1
            } else {
                acc
            }
        })
    });

    let vertical = transposed_grid.iter().fold(0, |acc, col| {
        acc + col.as_slice().windows(4).fold(0, |acc, window| {
            if window == b"XMAS" || window == b"SAMX" {
                acc + 1
            } else {
                acc
            }
        })
    });

    let diagonal = grid.as_slice().windows(4).fold(0, |acc, window| {
        let mut diags = 0;
        for i in 0..window[0].len() - 3 {
            let d1 = [
                window[0][i],
                window[1][i + 1],
                window[2][i + 2],
                window[3][i + 3],
            ];
            let d2 = [
                window[3][i],
                window[2][i + 1],
                window[1][i + 2],
                window[0][i + 3],
            ];
            if &d1 == b"XMAS" || &d1 == b"SAMX" {
                diags += 1;
            }
            if &d2 == b"XMAS" || &d2 == b"SAMX" {
                diags += 1;
            }
        }

        acc + diags
    });

    Some(horizontal + vertical + diagonal)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let mut a_indices = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'A' {
                a_indices.push((i, j));
            }
        }
    }

    let mas = a_indices.into_iter().fold(0, |acc, (i, j)| {
        if !(1..grid.len() - 1).contains(&i) || !(1..grid[0].len() - 1).contains(&j) {
            return acc;
        }

        let d1 = [grid[i - 1][j - 1], grid[i + 1][j + 1]];
        let d2 = [grid[i + 1][j - 1], grid[i - 1][j + 1]];

        if (&d1 == b"SM" || &d1 == b"MS") && (&d2 == b"SM" || &d2 == b"MS") {
            acc + 1
        } else {
            acc
        }
    });

    Some(mas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
