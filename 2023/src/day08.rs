use std::collections::{HashMap, HashSet};
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

fn part2_naive(lines: &str) -> usize {
    // This works fine for the example, but runs far too long for the real input, as I expected :-(

    let NavigationInput { instructions, nodes } = parse(lines);

    let initial_state: Vec<&str> = nodes.keys()
        .filter(|node| node.ends_with('A'))
        .map(|&node| node)
        .collect();

    instructions.iter().cycle()
        .scan(
            initial_state,
            |state, direction| {
                for node in state.iter_mut() {
                    let (left, right) = nodes.get(*node).unwrap();
                    *node = match direction {
                        Direction::Left => left,
                        Direction::Right => right
                    };
                }
                Some(state.iter()
                    .filter(|node| !node.ends_with('Z'))
                    .count())
            })
        .take_while_inclusive(|unfinished_node_count| *unfinished_node_count > 0)
        .count()
}

fn part2(lines: &str) -> usize {
    0
}

fn analyze_cycle(start_node: &str, navigation_input: &NavigationInput) -> Cycle {
    let NavigationInput { instructions, nodes } = navigation_input;

    // map (node, instruction index) to absolute index since start
    let mut seen: HashMap<(&str, usize), usize> = HashMap::new();
    let mut index = 0usize;
    let mut node = start_node;
    let mut instruction_index = 0usize;

    loop {
        seen.insert((node, instruction_index), index);
        let (left, right) = nodes.get(node).unwrap();
        let direction = instructions[instruction_index];
        match direction {
            Direction::Left => node = left,
            Direction::Right => node = right
        }
        instruction_index = (instruction_index + 1) % instructions.len();
        index += 1;

        if let Some(last_index) = seen.get(&(node, instruction_index)) {
            return Cycle {
                start: *last_index,
                length: index - *last_index,
                z_indices: vec![]
            }
        }
    }
}

#[derive(Debug)]
struct Cycle {
    start: usize,
    length: usize,
    z_indices: Vec<usize>
}

#[derive(Debug, Clone, Copy)]
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
    let re = Regex::new("([A-Z0-9]{3}) = \\(([A-Z0-9]{3}), ([A-Z0-9]{3})\\)").unwrap();

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

    const TEST_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part2() {
        assert_eq!(6, part2_naive(TEST_INPUT_3));
    }

    #[test]
    fn test() {
        println!("{:?}", analyze_cycle("11Z", &parse(TEST_INPUT_3)));
    }
}
