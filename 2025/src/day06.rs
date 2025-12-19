use std::cmp::min;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day06.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u64 {
    let input = parse(lines);

    (0..input.first().unwrap().len())
        .map(|i| {
            let mut numbers: Vec<u64> = Vec::new();
            for item in input.iter().map(|line| &line[i]) {
                match item.parse::<u64>() {
                    Ok(n) => numbers.push(n),
                    Err(_) => {
                        let func: &dyn Fn(u64, u64) -> u64 = match item.as_str() {
                            "+" => &add,
                            "*" => &mul,
                            _ => panic!("Invalid operation or number"),
                        };

                        return numbers.into_iter().reduce(func).unwrap();
                    }
                }
            }

            panic!("No operation found");
        })
        .sum()
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}

fn part2(lines: &str) -> u64 {
    // This works only if the first column is not empty
    assert!(lines.lines().any(|line| line.chars().next().unwrap() != ' '));

    let max_line_width = lines.lines().map(str::len).max().unwrap();

    // Append one column index to make sure that we have a final "empty" column
    let problem_column_ranges = (0..max_line_width + 1)
        // Create alternating groups of filled and empty column indices
        .chunk_by(|column_index|
            lines.lines()
                .any(|line| *line.as_bytes().get(*column_index).unwrap_or(&b' ') != b' '))
        .into_iter()
        // Keep only first column index per group
        .map(|(_non_empty, column_indices)| column_indices.into_iter().next().unwrap())
        // Build tuples, where the first and second item are the column indices
        // that a filled and empty column group start with, respectively
        .tuples()
        // Build Vec of filled column ranges
        .map(|(filled_columns_start, empty_columns_start)| filled_columns_start..empty_columns_start)
        .collect::<Vec<_>>();

    problem_column_ranges.into_iter()
        .map(|range| {
            let last_line = lines.lines().last().unwrap();
            let range_last_line = range.start..min(range.end, last_line.len());

            let func: &dyn Fn(u64, u64) -> u64 =
                match last_line[range_last_line]
                    .chars().into_iter()
                    .filter(|c| *c != ' ').next()
                {
                    Some('+') => &add,
                    Some('*') => &mul,
                    None => panic!("No operation found"),
                    _ => panic!("Invalid operation"),
                };

            range
                .map(|column_index|
                    lines.lines()
                        .map(|line| line.as_bytes().get(column_index).unwrap_or(&b' '))
                        .filter(|c| c.is_ascii_digit())
                        .map(|c| *c as char)
                        .join("")
                        .parse::<u64>()
                        .unwrap())
                .reduce(func)
                .unwrap()
        })
        .sum()
}

fn parse(lines: &str) -> Vec<Vec<String>> {
    lines.lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

    #[test]
    fn test_part1() {
        assert_eq!(4277556, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3263827, part2(TEST_INPUT));
    }
}
