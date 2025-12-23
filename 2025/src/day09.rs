use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day09.txt").expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> usize {
    let red_tiles = parse(lines);

    red_tiles.iter().enumerate()
        .flat_map(|(index, (x1, y1))|
            red_tiles.iter().take(index)
                .map(move |(x2, y2)|
                    ((x2 - x1).abs() as usize + 1) * ((y2 - y1).abs() as usize + 1)))
        .max().unwrap()
}

fn part2(lines: &str) -> i64 {
    0
}

fn parse(lines: &str) -> Vec<(i64, i64)> {
    lines.lines()
        .map(|line|
            line.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(50, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(25272, part2(TEST_INPUT));
    }
}
