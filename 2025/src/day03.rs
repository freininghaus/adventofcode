use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day03.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    parse(lines).iter()
        .map(|digits| largest_possible_joltage(digits))
        .sum()
}

fn part2(lines: &str) -> u64 {
    parse(lines).iter()
        .map(|digits| largest_possible_joltage_12_digits(digits))
        .sum()
}

fn largest_possible_joltage(digits: &[u8]) -> u32 {
    // 1st digit: maximum of all digits (except for the last one)
    let first_digit = *digits[..digits.len() - 1].iter().max().unwrap();

    // 2nd digit: maximum of all digits after the 1st digit
    let second_digit = *digits.iter()
        .skip_while(|d| **d != first_digit)
        .skip(1)
        .max()
        .unwrap();

    (10 * first_digit + second_digit) as u32
}

fn largest_possible_joltage_12_digits(digits: &[u8]) -> u64 {
    helper(digits, 12, 0)
}

fn helper(digits: &[u8], remaining_count: usize, acc: u64) -> u64 {
    if remaining_count == 0 {
        acc
    } else {
        let (first_digit, index) = digits[..digits.len() - remaining_count + 1].iter()
            .enumerate()
            .map(|(i, digit)| (digit, -(i as i64)))
            .max()
            .map(|(digit, index)| (digit, -index))
            .unwrap();

        helper(&digits[index as usize + 1..],
               remaining_count - 1,
               10 * acc + *first_digit as u64)
    }
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn parse(lines: &str) -> Vec<Vec<u8>> {
    lines.lines()
        .map(parse_line)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_larget_possible_joltage() {
        assert_eq!(98, largest_possible_joltage(&parse_line("987654321111111")));
        assert_eq!(89, largest_possible_joltage(&parse_line("811111111111119")));
        assert_eq!(78, largest_possible_joltage(&parse_line("234234234234278")));
        assert_eq!(92, largest_possible_joltage(&parse_line("818181911112111")));
    }

    #[test]
    fn test_part1() {
        assert_eq!(357, part1(TEST_INPUT));
    }

    #[test]
    fn test_larget_possible_joltage_12_digits() {
        assert_eq!(987654321111, largest_possible_joltage_12_digits(&parse_line("987654321111111")));
        assert_eq!(811111111119, largest_possible_joltage_12_digits(&parse_line("811111111111119")));
        assert_eq!(434234234278, largest_possible_joltage_12_digits(&parse_line("234234234234278")));
        assert_eq!(888911112111, largest_possible_joltage_12_digits(&parse_line("818181911112111")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3121910778619, part2(TEST_INPUT));
    }
}
