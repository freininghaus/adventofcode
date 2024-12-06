use std::collections::HashSet;
use std::iter::from_fn;
use std::ops::Add;

fn input() -> String {
    std::fs::read_to_string("input/day06.txt").expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Add<&Direction> for (i32, i32) {
    type Output = Self;
    fn add(self, rhs: &Direction) -> Self::Output {
        let (x, y) = self;

        match rhs {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

fn parse_dimensions(lines: &str) -> (i32, i32) {
    let height = lines.lines().count() as i32;
    let width = lines
        .lines()
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .count() as i32;

    (width, height)
}

fn parse_obstacles(lines: &str) -> HashSet<(i32, i32)> {
    lines
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}

fn parse_initial_state(lines: &str) -> ((i32, i32), Direction) {
    lines
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                match c {
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    '>' => Some(Direction::Right),
                    _ => None,
                }
                .map(|direction| ((x as i32, y as i32), direction))
            })
        })
        .next()
        .unwrap()
}

fn part1(lines: &str) -> usize {
    let (width, height) = parse_dimensions(lines);
    let obstacles = parse_obstacles(lines);

    let (mut pos, mut direction) = parse_initial_state(lines);

    from_fn(|| {
        let (x, y) = pos;

        if x < 0 || y < 0 || x == width || y == height {
            // position is outside the grid
            return None;
        }
        
        let new_pos = pos + &direction;
        
        if obstacles.contains(&new_pos) {
            // obstacle at new position -> stay at current position and turn right
            direction = direction.turn_right();
        } else {
            // update position
            pos = new_pos;
        }
        
        // yield the old position to make sure that the initial position is counted
        Some((x, y))
    })
        .collect::<HashSet<_>>()
        .len()
}

fn part2(_lines: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(41, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
