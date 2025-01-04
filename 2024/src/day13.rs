use itertools::Itertools;
use regex::Regex;

fn input() -> String {
    std::fs::read_to_string("input/day13.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

impl ClawMachine {
    fn parse(input: &str) -> ClawMachine {
        let regex = Regex::new(r"(Button [AB]|Prize): X[+=]([0-9]+), Y[+=]([0-9]+)").unwrap();

        let (button_a, button_b, prize) = input.lines()
            .map(|line| {
                let capture = regex.captures_iter(line)
                    .next().unwrap();
                let (_, [_, x, y]) = capture.extract();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect_tuple().unwrap();

        ClawMachine{ button_a, button_b, prize}
    }
}

fn parse(data: &str) -> Vec<ClawMachine> {
    data.split("\n\n")
        .map(|three_lines| ClawMachine::parse(three_lines))
        .collect()
}

fn tokens(machine: &ClawMachine) -> Option<i64> {
    // We could figure out how to combine the target coordinates using buttons A and B in better
    // ways, but the brute force approach is fast enough.
    (0..=100)
        .filter(|b| b * machine.button_b.0 <= machine.prize.0 && machine.button_b.1 <= machine.prize.1)
        .flat_map(|b| {
            let remainder = (
                machine.prize.0 - b * machine.button_b.0, 
                machine.prize.1 - b * machine.button_b.1);

            (0..=100)
                .filter(move |a| remainder.0 == a * machine.button_a.0 && remainder.1 == a * machine.button_a.1)
                .map(move |a| 3 * a + b)
        })
        .min()
}

fn part1(data: &str) -> i64 {
    parse(data).iter()
        .filter_map(tokens)
        .sum()
}

fn part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(480, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
