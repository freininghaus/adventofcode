use regex::Regex;

fn input() -> String {
    std::fs::read_to_string("input/day03.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    let mul_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    mul_regex.captures_iter(&lines)
        .map(|capture| capture.extract())
        .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum()
}

enum Instruction {
    Do,
    Dont,
    Mul(i32, i32)
}

fn part2(lines: &str) -> i32 {
    let regex = Regex::new(r"(do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();

    let instructions =
        regex.captures_iter(&lines)
            .map(|capture|
                match capture.get(0).unwrap().as_str() {
                    "do()" => Instruction::Do,
                    "don't()" => Instruction::Dont,
                    _ => Instruction::Mul(
                        capture.get(2).unwrap().as_str().parse().unwrap(),
                        capture.get(3).unwrap().as_str().parse().unwrap()
                    )
                });

    let (total, _) = instructions.fold((0, true),
        |(s, enabled), instruction|
            match instruction {
                Instruction::Do => (s, true),
                Instruction::Dont => (s, false),
                Instruction::Mul(a, b) =>
                    (if enabled { s + a * b } else { s }, enabled)
            });
    
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(TEST_INPUT));
    }

    const TEST_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(TEST_INPUT_2));
    }
}
