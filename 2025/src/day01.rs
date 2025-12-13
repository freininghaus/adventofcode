use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day01.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    parse(lines).iter()
        .scan(50, |angle, n| {
            *angle += n + 100;
            *angle %= 100;
            Some(*angle)
        })
        .filter(|&angle| angle == 0)
        .count() as u32
}

fn part2(lines: &str) -> u32 {
    // First item in the pair is the angle, second is the count of "zero crossings".
    parse(lines).iter()
        .scan((50, 0u32), |(angle, count), n| {
            let old_angle = *angle;
            *angle += n;

            let mut old = old_angle * n.signum();
            let mut new = *angle * n.signum();

            while (old < 0) {
                old += 100;
                new += 100;
            }

            *count += (new / 100 - old / 100) as u32;

            Some((*angle, *count))
        })
        .last()
        .unwrap()
        .1
}

fn parse(lines: &str) -> Vec<i32> {
    lines.lines()
        .map(|line| {
            let sign = match line.chars().next() {
                Some('L') => -1,
                Some('R') => 1,
                _ => panic!("Invalid input. Expected 'L' or 'R' at the start of every line, but found '{}'", line),
            };
            let number: i32 = line[1..].parse().expect("Invalid number in input");
            sign * number
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(TEST_INPUT));
    }
}
