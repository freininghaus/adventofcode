use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn input() -> String {
    std::fs::read_to_string("input/day19.txt").expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(data: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let (towels, designs) = data.split("\n\n").collect_tuple().unwrap();

    (
        towels
            .split(", ")
            .map(|towel| towel.as_bytes().to_vec())
            .collect(),
        designs
            .lines()
            .map(|design| design.as_bytes().to_owned())
            .collect(),
    )
}

// This works for the small test data set. However, it suffers from the problem that there are
// often multiple ways to build a given prefix of a design with towels. This leads to an exponential
// growth in the number of possibilities to consider when increasing the prefix length.
fn possible_design_naive(design: &[u8], towels: &Vec<Vec<u8>>) -> bool {
    design.is_empty()
        || towels.iter().any(|towel| {
            design
                .strip_prefix(towel.as_slice())
                .map(|rest| possible_design_naive(rest, towels))
                .unwrap_or(false)
        })
}

fn part1_naive(data: &str) -> usize {
    let (towels, designs) = parse(data);

    designs
        .iter()
        .filter(|design| possible_design_naive(&design, &towels))
        .count()
}

fn possible_design(design: &Vec<u8>, towels: &Vec<Vec<u8>>) -> bool {
    // Store lengths of prefixes of 'design' for which we know already that we can build them from
    // 'towels'.
    // In the following, we will remove the smallest item from this set repeatedly, check if the
    // next part of the design matches any towels, and then add the corresponding new prefix lengths.
    // This avoids the problem of the naive solution by analyzing each prefix only once, independent
    // of the way it was built from towels.
    let mut prefix_lengths = BTreeSet::from([0]);

    while !prefix_lengths.is_empty() {
        if prefix_lengths.contains(&design.len()) {
            return true;
        }

        let smallest = prefix_lengths.pop_first().unwrap();
        let suffix = &design[smallest..];

        for t in towels {
            if suffix.starts_with(t.as_slice()) {
                prefix_lengths.insert(smallest + t.len());
            }
        }
    }

    false
}

fn part1(data: &str) -> usize {
    let (towels, designs) = parse(data);

    designs
        .iter()
        .filter(|design| possible_design(&design, &towels))
        .count()
}

fn ways_to_make_design(design: &Vec<u8>, towels: &Vec<Vec<u8>>) -> usize {
    let mut prefix_lengths = BTreeMap::from([(0, 1)]);

    while !prefix_lengths.is_empty() {
        let (smallest_prefix_length, ways) = prefix_lengths.pop_first().unwrap();
        if smallest_prefix_length == design.len() {
            return ways;
        }

        let suffix = &design[smallest_prefix_length..];

        for t in towels {
            if suffix.starts_with(t.as_slice()) {
                prefix_lengths.entry(smallest_prefix_length + t.len())
                    .and_modify(|result| *result += ways)
                    .or_insert(ways);
            }
        }
    }

    0
}

fn part2(data: &str) -> usize {
    let (towels, designs) = parse(data);

    designs
        .iter()
        .map(|design| ways_to_make_design(&design, &towels))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!(6, part1_naive(TEST_INPUT));
        assert_eq!(6, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
