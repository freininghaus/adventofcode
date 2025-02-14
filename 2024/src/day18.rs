use itertools::Itertools;
use crate::utils::{point_sets_map, Dimensions, Point, PointSet};

#[path = "../utils/pointset.rs"]
mod utils;

fn input() -> String {
    std::fs::read_to_string("input/day18.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn fallen_bytes(data: &str) -> Vec<(usize, usize)> {
    data
        .lines()
        .map(
            |line|
                line.split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect_tuple()
                    .unwrap()
        )
        .collect()
}

// Find out if we are on the 71x71 grid or on the 7x7 test grid
fn is_test_grid(positions: &Vec<(usize, usize)>) -> bool {
    positions.iter().all(
        |(x, y)| *x <= 6 && *y <= 6
    )
}

fn grid_dimensions(positions: &Vec<(usize, usize)>) -> Dimensions {
    let size = if is_test_grid(positions) {
        7
    } else {
        71
    };

    Dimensions{ width: size + 1, height: size}
}

// We add a blocked column at the right to prevent wrapping
fn blocked_right_column(positions: &Vec<(usize, usize)>) -> PointSet {
    let dimensions = grid_dimensions(positions);

    PointSet::from_points(
        &dimensions,
        (0..dimensions.height)
            .map(|y| Point { x: dimensions.width - 1, y })
    )
}

fn start_and_exit(dimensions: &Dimensions) -> (PointSet, PointSet) {
    (
        PointSet::from_point(
            &dimensions,
            &Point { x: 0, y: 0 }
        ),
        PointSet::from_point(
            &dimensions,
            &Point {
                x: dimensions.width - 2,  // ignore the blocked rightmost column
                y: dimensions.height - 1
            }
        )
    )
}

// In part 1, we take only some of the falled bytes into account
fn corrupted_locations(data: &str) -> PointSet {
    let fallen_bytes = fallen_bytes(data);

    let blocked_column = blocked_right_column(&fallen_bytes);

    &PointSet::from_points(
        &blocked_column.dimensions,
        fallen_bytes.iter()
            .take(if is_test_grid(&fallen_bytes) { 12 } else { 1024 })
            .map(|(x, y)| Point { x: *x, y: *y })
    ) | &blocked_column
}

fn part1(data: &str) -> usize {
    let blocked = corrupted_locations(data);
    let safe = !&blocked;

    let (start, exit) = start_and_exit(&blocked.dimensions);

    (1usize..).scan(start, |visited, step| {
        *visited |= &(visited.shake() & &safe);
        let reached_exit = !(&*visited & &exit).is_empty();

        /*
        println!("{}\n\n", point_sets_map(&vec![
            ('#', &blocked),
            ('x', &*visited)
        ]));
        */

        Some((reached_exit, step))
    })
        .skip_while(|(reached_exit, _)| !reached_exit)
        .map(|(_, step)| step)
        .next()
        .unwrap()
}

fn part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        assert_eq!(22, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
