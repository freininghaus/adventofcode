use std::collections::BTreeMap;
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

fn checksum(disk_map: &[Option<usize>]) -> usize {
    disk_map.iter().enumerate()
        .map(|(index, item)| {
            item.unwrap_or(0) * index
        })
        .sum()
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

    checksum(&disk_map)
}

fn part2(data: &str) -> usize {
    let mut disk_map = parse(data);

    // (start index, length, common value)
    let spans: Vec<(usize, usize, Option<usize>)> = disk_map.iter()
        .enumerate()
        .chunk_by(|(_index, value)| *value)
        .into_iter()
        .map(|(key, chunk)| (chunk.count(), key))
        .scan(0, |index, (chunk_size, value)| {
            let start_index = *index;
            *index += chunk_size;

            Some((start_index, chunk_size, *value))
        })
        .collect();

    // (start index, length) of free spans
    let mut free_spans: BTreeMap<usize, usize> = spans.iter()
        .filter(|(_, _, value)| value.is_none())
        .map(|(start_index, chunk_size, _)| (*start_index, *chunk_size))
        .collect();

    // (start index, length) of files to move
    let files_to_move: BTreeMap<usize, usize> = spans.iter()
        .filter(|(_, _, value)| value.is_some())
        .map(|(start_index, chunk_size, _)| (*start_index, *chunk_size))
        .collect();

    for (file_start_index, file_size) in files_to_move.iter().rev() {
        let target_span = free_spans.iter()
            .map(|(start_index, span_size)| (*start_index, *span_size))
            .take_while(|(start_index, _)| start_index < file_start_index)
            .filter(|(_, span_size)| span_size >= file_size)
            .next();

        if let Some((target_span_start_index, target_span_size)) = target_span {
            // move file
            for i in 0..*file_size {
                disk_map[target_span_start_index + i] = disk_map[file_start_index + i];
                disk_map[file_start_index + i] = None;
            }

            // update free spans
            free_spans.remove(&target_span_start_index);
            if target_span_size > *file_size {
                // free span has not been used completely
                free_spans.insert(
                    target_span_start_index + *file_size,
                    target_span_size - *file_size
                );
            }
        }

    }

    checksum(&disk_map)
}

fn print_disk_map(disk_map: &[Option<usize>]) {
    for value in disk_map {
        match value {
            Some(value) => print!("{}", value),
            None => print!(".")
        }
    }

    println!();
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
        assert_eq!(2858, part2(TEST_INPUT));
    }
}
