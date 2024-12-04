use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day01.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    let (mut left, mut right) = parse_lists(lines);
    left.sort();
    right.sort();

    left.iter().zip(right.iter())
        .map(|(l, r)| (l - r).abs() as u32)
        .sum()
}

fn part2(_lines: &str) -> i32 {
    let (left, right) = parse_lists(_lines);

    let right_list_frequency = right.iter().counts();

    left.iter()
        .map(|x| 
            x * *right_list_frequency
                .get(&x)
                .unwrap_or(&0)
                as i32)
        .sum()
}

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines()
        .map(|l|
            l.split_whitespace().map(str::parse::<i32>).map(Result::unwrap).collect_tuple().unwrap())
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, part2(TEST_INPUT));
    }
}
