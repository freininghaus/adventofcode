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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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

#[derive(Clone)]
struct Map<'a> {
    width: i32,
    height: i32,
    obstacles: &'a HashSet<(i32, i32)>,
    new_obstacle: Option<(i32, i32)>
}

fn single_step(map: &Map, pos: (i32, i32), direction: Direction) -> Option<((i32, i32), Direction)> {
    let (x, y) = pos;

    if x < 0 || y < 0 || x == map.width || y == map.height {
        // position is outside the grid
        return None;
    }

    let new_pos = pos + &direction;

    if map.obstacles.contains(&new_pos)
        || map.new_obstacle.map(|o| o == new_pos) == Some(true) {
        // obstacle at new position -> stay at current position and turn right
        Some((pos, direction.turn_right()))
    } else {
        // update position
        Some((new_pos, direction))
    }
}

fn visited_positions(map: &Map, mut pos: (i32, i32), mut direction: Direction) -> Vec<(i32, i32)> {
    let initial_position = pos;

    from_fn(|| {
        let (new_pos, new_direction) = single_step(map, pos, direction)?;
        pos = new_pos;
        direction = new_direction;
        Some(pos)
    })
        .filter(|&p| p != initial_position)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn stuck_in_loop(map: &Map, mut pos: (i32, i32), mut direction: Direction) -> bool {
    let mut seen_positions: HashSet<((i32, i32), Direction)> = HashSet::new();

    loop {
        let Some((new_pos, new_direction)) = single_step(map, pos, direction) else {
            // we have moved out of the map
            return false;
        };

        if seen_positions.contains(&(new_pos, new_direction)) {
            return true;
        }

        seen_positions.insert((new_pos, new_direction));

        pos = new_pos;
        direction = new_direction;
    }
}

fn part2(lines: &str) -> usize {
    let (width, height) = parse_dimensions(lines);
    let obstacles = parse_obstacles(lines);
    let (initial_pos, initial_direction) = parse_initial_state(lines);

    let map = Map {
        width: width,
        height: height,
        obstacles: &obstacles,
        new_obstacle: None
    };

    let candidates = visited_positions(&map, initial_pos, initial_direction);

    candidates.iter()
        .map(|candidate| {
            let mut new_map = map.clone();
            new_map.new_obstacle = Some(*candidate);
            stuck_in_loop(&new_map, initial_pos, initial_direction)
        })
        .filter(|is_stuck| *is_stuck)
        .count()
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
        assert_eq!(6, part2(TEST_INPUT));
    }
}
