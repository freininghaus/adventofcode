use std::collections::HashSet;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day04.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> usize {
    let roll_positions = parse(lines);
    roll_positions.iter()
        .filter(|pos|
            neighbors(pos).iter()
                .filter(|neighbor| roll_positions.contains(*neighbor))
                .count() < 4
        )
    .count()
}

fn part2(lines: &str) -> usize {
    (0..).scan(parse(lines), |roll_positions, _| {
        let removable_roll_positions =  accessible_for_forklift(roll_positions);
        if removable_roll_positions.is_empty() {
            return None;
        }

        for pos in &removable_roll_positions {
            roll_positions.remove(&pos);
        }

        Some(removable_roll_positions.len() as usize)
    }).sum()
}

fn neighbors(&(x, y): &(i32, i32)) -> Vec<(i32, i32)> {
    (-1..=1).flat_map(
        |dx| (-1..=1)
            .filter(move |dy| dx != 0 || *dy != 0)
            .map(
            move |dy| (x + dx, y + dy)
        )
    ).collect()
}

fn accessible_for_forklift(roll_positions: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    roll_positions.iter()
        .filter(|pos|
            neighbors(pos).iter()
                .filter(|neighbor| roll_positions.contains(neighbor))
                .count() < 4)
        .map(|pos| pos.clone())
        .collect()
}

fn parse(lines: &str) -> HashSet<(i32, i32)> {
    lines.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '@')
            .map(move |(x, _)| (x as i32, y as i32))
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(43, part2(TEST_INPUT));
    }
}
