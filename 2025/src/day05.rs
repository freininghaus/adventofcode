use std::cmp::max;
use std::ops::RangeInclusive;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day05.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> usize {
    let (fresh_ranges, available_ingredients) = parse(lines);

    available_ingredients.iter()
        .filter(
            |id| fresh_ranges.iter().any(|range| range.contains(id))
        )
        .count()
}

fn part2(lines: &str) -> usize {
    let mut fresh_ranges = parse(lines).0;
    fresh_ranges.sort_by(|a, b| a.start().cmp(&b.start()));

    let mut result: Vec<RangeInclusive<usize>> = Vec::new();
    for range in fresh_ranges {
        if result.is_empty() {
            result.push(range);
            continue;
        }

        let last = result.last().unwrap();

        if range.start() <= last.end() {
            let last = result.pop().unwrap();
            result.push(*last.start()..=max(*last.end(), *range.end()));
        } else {
            result.push(range);
        }
    }

    result.iter().map(|range| range.end() + 1 - range.start()).sum()
}

fn parse(lines: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (fresh_ranges, available) = lines.split_once("\n\n").unwrap();

    (fresh_ranges.lines()
        .map(|line| line.split("-").map(|n| n.parse().unwrap()).tuples().next().unwrap())
        .map(|(a, b)| a..=b)
        .collect(),
     //
     available.lines().map(str::parse).map(Result::unwrap).collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(14, part2(TEST_INPUT));
    }
}
