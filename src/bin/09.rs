advent_of_code::solution!(9);

#[derive(Debug)]
enum Block {
    Empty,
    File { id: u64 },
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut id = 0;
    for (i, c) in input.chars().enumerate() {
        // editor keeps re-adding the newline, crashing the program
        if c == '\n' {
            break;
        }

        let count = c.to_digit(10).unwrap() as u8;
        if i % 2 == 0 {
            for _ in 0..count {
                blocks.push(Block::File { id });
            }
            id += 1;
        } else {
            for _ in 0..count {
                blocks.push(Block::Empty);
            }
        }
    }
    blocks
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse_input(input);

    let mut head = 0;
    let mut tail = blocks.len() - 1;

    loop {
        while !matches!(blocks[head], Block::Empty) {
            head += 1;
        }

        while matches!(blocks[tail], Block::Empty) {
            tail -= 1;
        }

        if tail <= head {
            break;
        }

        blocks.swap(head, tail);
    }

    Some(
        blocks
            .into_iter()
            .filter_map(|b| match b {
                Block::Empty => None,
                Block::File { id } => Some(id),
            })
            .enumerate()
            .fold(0, |acc, (i, id)| acc + (i as u64 * id)),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks = parse_input(input);

    // (index, size)
    let mut free_blocks = Vec::new();
    let mut current_start = None;
    let mut current_size = 0;
    for (i, block) in blocks.iter().enumerate() {
        match (block, current_start) {
            (Block::File { .. }, None) => (),
            (Block::File { .. }, Some(index)) => {
                free_blocks.push((index, current_size));
                current_start = None;
                current_size = 0;
            }
            (Block::Empty, None) => {
                current_start = Some(i);
                current_size += 1;
            }
            (Block::Empty, Some(_)) => {
                current_size += 1;
            }
        }
    }

    // (index, size)
    let mut file_blocks = Vec::new();
    let mut current_start = None;
    let mut current_file = None;
    let mut current_size = 0;
    for (i, block) in blocks.iter().enumerate() {
        match (block, current_start) {
            (Block::Empty, None) => current_file = None,
            (Block::Empty, Some(index)) => {
                file_blocks.push((index, current_size));
                current_start = None;
                current_size = 0;
                current_file = None;
            }
            (Block::File { id }, None) => {
                current_start = Some(i);
                current_size += 1;
                current_file = Some(id);
            }
            (Block::File { id }, Some(_)) if current_file.is_some_and(|f| f == id) => {
                current_size += 1;
            }
            (Block::File { id }, Some(index)) => {
                file_blocks.push((index, current_size));
                current_start = Some(i);
                current_file = Some(id);
                current_size = 1;
            }
        }
    }

    if current_file.is_some() {
        file_blocks.push((current_start.unwrap(), current_size));
    }

    'outer: for (index, size) in file_blocks.into_iter().rev() {
        for i in 0..free_blocks.len() {
            let (free_index, free_size) = free_blocks[i];

            if free_index >= index {
                continue 'outer;
            }

            if free_size >= size {
                for x in 0..size {
                    blocks.swap(free_index + x, index + x);
                }

                if free_size > size {
                    free_blocks[i] = (free_index + size, free_size - size);
                } else {
                    free_blocks.remove(i);
                }
                continue 'outer;
            }
        }
    }

    Some(
        blocks
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                Block::Empty => None,
                Block::File { id } => Some((i, id)),
            })
            .fold(0, |acc, (i, id)| acc + (i as u64 * id)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.unwrap() < 8423896686086);
    }
}
