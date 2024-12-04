use std::str::FromStr;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day02.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> usize {
    parse_reports(lines)
        .filter(is_safe)
        .count()
}

fn part2(lines: &str) -> usize {
    parse_reports(lines)
        .filter(is_safe_if_dampened)
        .count()
}

fn parse_reports(input: &str) -> impl Iterator<Item=Report> + '_ {
    input.lines().map(|l| l.parse().unwrap())
}

struct Report(Vec<i32>);

fn is_safe(report: &Report) -> bool {
    let Report(levels) = report;

    if levels.iter()
        .tuple_windows()
        .any(|(a, b)| 
            a == b || (a - b).abs() > 3) {
        return false;
    }
    
    levels.is_sorted() || levels.iter().rev().is_sorted()
}

fn is_safe_if_dampened(report: &Report) -> bool {
    let Report(levels) = report;
    
    is_safe(&report) || 
        (0..levels.len())
            .map(|skip_index| levels.iter()
                .enumerate()
                .filter(|(index, _)| *index != skip_index)
                .map(|(_, x)| *x)
                .collect::<Vec<_>>()
            )
            .map(Report)
            .any(|report| is_safe(&report))
}

impl FromStr for Report {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_whitespace()
            .map(&str::parse)
            .map(Result::unwrap)
            .collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(TEST_INPUT));
    }
}
