use std::collections::HashMap;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day11.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<u64> {
    data.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn single_stone_blink(stone: &u64) -> Vec<u64> {
    if *stone == 0 {
        return vec![1];
    }

    let stone_str = stone.to_string();
    if stone_str.bytes().count() % 2 == 0 {
        let half_len = stone_str.len() / 2;
        return vec![stone_str[..half_len].parse().unwrap(),
                    stone_str[half_len..].parse().unwrap()];
    }

    vec![stone * 2024]
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones.iter()
        .flat_map(single_stone_blink)
        .collect()
}

fn part1(data: &str) -> usize {
    let mut stones = parse(data);
    for _ in 0..25 {
        stones = blink(&stones);
    }
    stones.len()
}

fn blink_groups(stone_counts: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut result: HashMap<u64, usize> = HashMap::new();

    for (stone, count) in stone_counts {
        for new_stone in single_stone_blink(stone) {
            *result.entry(new_stone).or_insert(0) += count;
        }
    }

    result
}

fn part2(data: &str) -> usize {
    let mut stone_counts: HashMap<u64, usize> = parse(data).iter()
        .sorted()
        .chunk_by(|x| **x)
        .into_iter()
        .map(|(stone, chunk)| (stone, chunk.count()))
        .collect();

    for _ in 0..75 {
        stone_counts = blink_groups(&stone_counts);
    }

    stone_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_blink() {
        let data_0 = parse(&TEST_INPUT);
        assert_eq!(data_0, vec![125, 17]);

        let data_1 = blink(&data_0);
        assert_eq!(data_1, vec![253000, 1, 7]);

        let data_2 = blink(&data_1);
        assert_eq!(data_2, vec![253, 0, 2024, 14168]);

        let data_3 = blink(&data_2);
        assert_eq!(data_3, vec![512072, 1, 20, 24, 28676032]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(55312, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        // no given result for part 2 with test input
        //assert_eq!(42, part2(TEST_INPUT));
    }
}
