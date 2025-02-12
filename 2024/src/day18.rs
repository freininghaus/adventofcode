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

fn corrupted_locations(fallen_bytes: &str) -> PointSet {
    let fallen_bytes: Vec<(usize, usize)> = fallen_bytes
        .lines()
        .map(
            |line|
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap()
        )
        .collect();

    // Find out if we are on the 71x71 grid or on the 7x7 test grid
    let test_grid = fallen_bytes.iter()
        .all(|(x, y)| *x <= 6 && *y <= 6);

    let size = if test_grid { 7 } else { 71 };

    // We add a blocked column at the right to prevent wrapping
    let dimensions = Dimensions{ width: size + 1, height: size};

    let blocked_column = PointSet::from_points(
        &dimensions,
        (0..size)
            .map(|y| Point { x: size, y })
    );

    &PointSet::from_points(
        &dimensions,
        fallen_bytes.into_iter()
            .take(if test_grid { 12 } else { 1024 })
            .map(|(x, y)| Point { x, y })
    ) | &blocked_column
}

fn part1(data: &str) -> usize {
    let blocked = corrupted_locations(data);

    let start = PointSet::from_point(
        &blocked.dimensions,
        &Point { x: 0, y: 0 }
    );

    // Do not use the width here - note that we have added a blocked column at the right edge.
    let size = blocked.dimensions.height;

    let exit = PointSet::from_point(
        &blocked.dimensions,
        &Point { x: size - 1, y: size - 1 }
    );

    let safe = !&blocked;

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
