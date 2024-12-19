use std::collections::HashSet;

fn input() -> String {
    std::fs::read_to_string("input/day10.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|line|
            line.chars()
                .map(|c| c
                    .to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn neighbors(map: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let width = map[0].len();
    let height = map.len();

    [
        (Some(x), y.checked_sub(1)),
        (Some(x), Some(y + 1)),
        (Some(x + 1), Some(y)),
        (x.checked_sub(1), Some(y))
    ].iter()
        .filter(|(x, y)| x.is_some() && y.is_some())
        .map(|(x, y)| (x.unwrap(), y.unwrap()))
        .filter(|(x, y)| x < &width && y < &height)
        .collect()
}

fn reachable_9_height_positions(map: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> HashSet<(usize, usize)> {
    let height = map[y][x];

    if height == 9 {
        return vec![(x, y)].into_iter().collect();
    }

    neighbors(map, (x, y)).iter()
        .filter(|&&(x, y)| map[y][x] == height + 1)
        .flat_map(|&(x, y)| reachable_9_height_positions(&map, (x, y)))
        .collect()
}

fn part1(data: &str) -> usize {
    let map = parse(&data);
    let map_ref = &map;
    map.iter()
        .enumerate()
        .flat_map(
            |(y, row)|
                row.iter()
                    .enumerate()
                    .filter(|(_, height)| height == &&0)
                    .map(move |(x, _)| reachable_9_height_positions(map_ref, (x, y)).len())
        )
        .sum()
}

fn part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(36, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
