use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day07.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> usize {
    let (start, all_splitters) = parse(lines);

    all_splitters.iter().scan(HashSet::from([start]), |beams, splitters| {
        let split_beams = beams.iter()
            .filter(|beam| splitters.contains(beam))
            .map(|beam| *beam)
            .collect_vec();

        for beam in split_beams.iter() {
            beams.remove(&beam);
            beams.insert(beam - 1);
            beams.insert(beam + 1);
        }

        Some(split_beams.len())
    })
    .sum()
}

fn part2(lines: &str) -> usize {
    let (start, all_splitters) = parse(lines);

    // Maps (row, column) to different timelines stating at this position
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    fn count_timelines_starting_at(cache: &mut HashMap<(usize, usize), usize>,
                                   all_splitters: &Vec<HashSet<usize>>,
                                   (row, column): (usize, usize)) -> usize {
        if let Some(result) = cache.get(&(row, column)) {
            return *result;
        }

        let result = if row >= all_splitters.len() {
            1
        } else if all_splitters[row].contains(&column) {
            count_timelines_starting_at(cache, all_splitters, (row + 1, column - 1)) +
                count_timelines_starting_at(cache, all_splitters, (row + 1, column + 1))
        } else {
            count_timelines_starting_at(cache, all_splitters, (row + 1, column))
        };

        cache.insert((row, column), result);
        result
    }

    count_timelines_starting_at(&mut cache, &all_splitters, (0, start))
}

fn parse(lines: &str) -> (usize, Vec<HashSet<usize>>) {
    let mut lines = lines.lines();

    (
        lines.next().unwrap().chars().enumerate()
            .filter(|(_, c)| *c == 'S')
            .next().unwrap()
            .0,
        //
        lines
            .map(|line|
                line.chars().enumerate()
                    .filter(|(_, c)| *c == '^')
                    .map(|(index, _)| index)
                    .collect())
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(40, part2(TEST_INPUT));
    }
}
