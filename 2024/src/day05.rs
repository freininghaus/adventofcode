use std::cmp::Ordering;
use std::collections::HashSet;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day05.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(lines: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let Some((ordering_rules, updates)) = lines.split("\n\n").collect_tuple() else {
        panic!("Could not parse input");
    };

    (
        ordering_rules.lines()
            .map(|line| line.split('|')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap())
            .collect(),
        updates.lines()
            .map(|line| line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect())
            .collect()
    )
}

fn ordered_correctly(ordering_rules: &HashSet<(u32, u32)>, update: &Vec<u32>) -> bool {
    (0..update.len() - 1).all(
        |i| (i + 1 .. update.len()).all(
            |j| !ordering_rules.contains(&(update[j], update[i]))
        )
    )
}

fn part1(lines: &str) -> u32 {
    let (ordering_rules, updates) = parse(&lines);

    updates.iter()
        .filter(|update| ordered_correctly(&ordering_rules, &update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn reorder(ordering_rules: &HashSet<(u32, u32)>, update: &Vec<u32>) -> Vec<u32> {
    update.iter()
        .sorted_by(|a, b|
            if ordering_rules.contains(&(**a, **b)) {
                Ordering::Less
            } else if ordering_rules.contains(&(**b, **a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        )
        .map(|a| *a)
        .collect()
}

fn part2(lines: &str) -> u32 {
    let (ordering_rules, updates) = parse(&lines);

    updates.iter()
        .filter(|update| !ordered_correctly(&ordering_rules, &update))
        .map(|update| reorder(&ordering_rules, &update))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(143, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, part2(TEST_INPUT));
    }
}
