use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day07.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(lines: &str) -> Vec<(u64, Vec<u64>)> {
    lines.lines()
        .map(|line| line.split(':').collect_tuple().unwrap())
        .map(|(result, numbers)|
            (result.parse().unwrap(),
             numbers.split_whitespace()
                 .map(str::parse)
                 .map(Result::unwrap)
                 .collect()))
        .collect()
}

fn could_be_true_1(result: u64, numbers: &[u64]) -> bool {
    if numbers.len() == 1 {
        return result == numbers[0];
    }

    let last = numbers[numbers.len() - 1];
    let rest = &numbers[0..(numbers.len() - 1)];

    (result >= last && could_be_true_1(result - last, rest))
        || (result % last == 0 && could_be_true_1(result / last, rest))
}

fn part1(lines: &str) -> u64 {
    parse(lines).iter()
        .filter(|(result, numbers)| could_be_true_1(*result, numbers))
        .map(|(result, _)| result)
        .sum()
}

fn remove_suffix(n: u64, suffix: u64) -> Option<u64> {
    let n_str = n.to_string();
    let suffix_str = suffix.to_string();

    if !n_str.ends_with(suffix_str.as_str()) {
        return None;
    }

    if n_str.len() == suffix_str.len() {
        return None;
    }

    Some(n_str[0..n_str.len() - suffix_str.len()].parse().unwrap())
}

fn could_be_true_2(result: u64, numbers: &[u64]) -> bool {
    if numbers.len() == 1 {
        return result == numbers[0];
    }

    let last = numbers[numbers.len() - 1];
    let rest = &numbers[0..(numbers.len() - 1)];

    (result >= last && crate::could_be_true_2(result - last, rest))
        || (result % last == 0 && crate::could_be_true_2(result / last, rest))
        ||
        {
            if let Some(new_result) = remove_suffix(result, last) {
                crate::could_be_true_2(new_result, rest)
            } else {
                false
            }
        }
}

fn part2(lines: &str) -> u64 {
    parse(lines).iter()
        .filter(|(result, numbers)| could_be_true_2(*result, numbers))
        .map(|(result, _)| result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(3749, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(11387, part2(TEST_INPUT));
    }
}
