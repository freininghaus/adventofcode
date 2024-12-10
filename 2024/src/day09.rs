use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day09.txt").expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<Option<usize>> {
    data.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(index, mut lengths)| -> Vec<Option<usize>> {
            let file_length = lengths.next().unwrap();
            let file_blocks = std::iter::repeat(Some(index)).take(file_length);

            if let Some(free_space_length) = lengths.next() {
                let free_blocks = std::iter::repeat(None).take(free_space_length);
                file_blocks.chain(free_blocks).collect()
            } else {
                file_blocks.collect()
            }
        })
        .collect()
}

fn part1(data: &str) -> usize {
    let mut disk_map = parse(data);

    let mut index = 0;
    while index < disk_map.len() {
        if disk_map[index].is_none() {
            let last = disk_map.pop().unwrap();
            disk_map[index] = last;

            while disk_map.last().unwrap().is_none() {
                disk_map.pop();
            }
        }
        index += 1;
    }

    disk_map.iter().enumerate()
        .map(|(index, item)| {
            item.unwrap_or(0) * index
        })
        .sum()
}

fn part2(data: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse() {
        // examples from problem description
        assert_eq!(
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ],
            parse("12345")
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(1928, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
