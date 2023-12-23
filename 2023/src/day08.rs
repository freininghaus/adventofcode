use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn input() -> String {
    std::fs::read_to_string("input/day08.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> usize {
    let NavigationInput { instructions, nodes } = parse(lines);

    instructions.iter().cycle()
        .scan(
            "AAA",
            |node, direction| {
                let (left, right) = nodes.get(*node).unwrap();
                *node = match direction {
                    Direction::Left => left,
                    Direction::Right => right
                };
                Some(*node)
            })
        .take_while_inclusive(|&node| node != "ZZZ")
        .count()
}

fn part2(_lines: &str) -> u32 {
    0
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct NavigationInput<'a> {
    instructions: Vec<Direction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>
}

fn parse(lines: &str) -> NavigationInput {
    let mut lines = lines.lines();

    let instructions = lines.next().unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("unknown direction")
        })
        .collect();

    assert_eq!(lines.next(), Some(""));

    // after we did so much with nom this year, maybe it's time to practice regular expressions again.
    let re = Regex::new("([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)").unwrap();

    let nodes = lines
        .map(|line| {
            // Note that we want to store references to the original input in the HashMap, hence this complexity here.
            let caps = re.captures(line).unwrap();
            (caps.get(1).unwrap().as_str(),
             (caps.get(2).unwrap().as_str(),
              caps.get(3).unwrap().as_str()))
        })
        .collect();

    NavigationInput {
        instructions,
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(TEST_INPUT_1));
        assert_eq!(6, part1(TEST_INPUT_2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT_1));
    }
}
