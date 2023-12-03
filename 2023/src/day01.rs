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
    lines.lines()
        .map(line_value_1)
        .sum()
}

fn line_value_1(line: &str) -> u32 {
    let digits = line.chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    assert!(digits.len() > 0);
    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn part2(lines: &str) -> u32 {
    lines.lines()
        .map(line_value_2)
        .sum()
}

fn line_value_2(line: &str) -> u32 {
    // Working with bytes is easier because we can easily build slices of the line then.
    let line = line.as_bytes();

    let digits_values: Vec<_> = (0u32..=9)
        .map(|d| (d.to_string().bytes().collect::<Vec<_>>(), d))
        .collect();
    let words_values: Vec<_> = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        .map(|word| word.as_bytes().into_iter().map(|&c| c).collect::<Vec<u8>>())
        .into_iter().zip(1u32..=9)
        .collect();

    let all = [digits_values, words_values].concat();

    let digits_in_line: Vec<u32> = (0..line.len())
        .flat_map(|i|
            all.iter().flat_map(move |(needle, value)|
                if line[i..].starts_with(needle) {
                    Some(*value)
                } else {
                    None
                }
            )
        ).collect();

    assert!(digits_in_line.len() > 0);
    10 * digits_in_line.first().unwrap() + digits_in_line.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part1() {
        assert_eq!(142, part1(TEST_INPUT_1));
    }

    const TEST_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part2() {
        assert_eq!(281, part2(TEST_INPUT_2));
    }

}