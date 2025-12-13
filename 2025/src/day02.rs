use std::collections::HashSet;
use std::ops::RangeInclusive;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day02.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u64 {
    parse(lines).iter()
        .map(|range| range.clone().filter(|n| is_invalid1(*n)).sum::<u64>())
        .sum()
}

fn part2(lines: &str) -> u64 {
    parse(lines).iter()
        .map(|range| range.clone().filter(|n| is_invalid2(*n)).sum::<u64>())
        .sum()
}

fn is_invalid1(n: u64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();
    b[..b.len() / 2] == b[b.len() / 2..]
}

fn is_invalid2(n: u64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();

    for chunk_count in 2..=b.len() {
        if b.len() % chunk_count != 0 {
            continue;
        }

        if b.chunks(b.len() / chunk_count).all_equal() {
            return true;
        }
    }

    false
}

fn parse(lines: &str) -> Vec<RangeInclusive<u64>> {
    lines.lines().join("").split(',')
        .map(|s| s.split('-').map(|n| n.parse().unwrap()).tuples().next().unwrap())
        .map(|(a, b)| a..=b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(1227775554, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4174379265, part2(TEST_INPUT));
    }
}
