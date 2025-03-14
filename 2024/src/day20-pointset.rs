use crate::utils::{parse_point_sets, Dimensions, Direction, Point, PointSet};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::successors;

#[path = "../utils/pointset.rs"]
mod utils;

fn input() -> String {
    std::fs::read_to_string("input/day20.txt").expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

struct Map {
    walls: PointSet,
    track: PointSet,
    start: PointSet,
    end: PointSet,
}

fn parse(data: &str) -> Map {
    let mut point_sets = parse_point_sets(data);

    let walls = point_sets.remove(&'#').unwrap();
    let start = point_sets.remove(&'S').unwrap();
    let end = point_sets.remove(&'E').unwrap();

    let track = &point_sets.remove(&'.').unwrap() | &start | &end;

    Map {
        walls,
        track,
        start,
        end,
    }
}

fn shortest_path_length(map: &Map) -> usize {
    successors(Some(map.start.clone()), |current| {
        let new = &current.shake() & &map.track;

        if (&new & &map.end).is_empty() {
            Some(new)
        } else {
            None
        }
    })
    .count()
}

fn shortest_path_length_with_cheat(
    map: &Map,
    cheat_pos: &Point,
    cheat_direction: Direction,
) -> usize {
    let cheat_pos = PointSet::from_point(&map.walls.dimensions, cheat_pos);
    let cheat_track = &map.track | &cheat_pos; // allow to enter the first cheat position

    successors(Some(map.start.clone()), |current| {
        // allow to leave the first cheat position only into the given direction
        let cheat = (current & &cheat_pos).shift(cheat_direction);

        let new = &(&(current & &map.track).shake() & &cheat_track) | &cheat;

        if (&new & &map.end).is_empty() {
            Some(new)
        } else {
            None
        }
    })
    .count()
}

fn cheats(map: &Map) -> Vec<(Point, Direction)> {
    // Find pairs of point and direction where
    // - point is a wall cell
    // - the neighbouring in the given direction is not a wall cell
    (1..map.walls.dimensions.width - 1)
        .flat_map(|x| (1..map.walls.dimensions.height - 1).map(move |y| Point { x, y }))
        .filter(|p| map.walls.contains(p))
        .flat_map(|p| {
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .map(|direction| (p, direction))
        })
        .filter(|(p, direction)| !map.walls.contains(&(*p + *direction)))
        .collect()
}

// For each possible time reduction, count how many distinct cheats make it possible.
// This works for the test map, but is too slow for the real map.
fn cheat_counts_naive(data: &str) -> HashMap<usize, usize> {
    let map = parse(data);
    let base_time = shortest_path_length(&map);

    cheats(&map)
        .iter()
        .map(|(p, direction)| base_time - shortest_path_length_with_cheat(&map, p, *direction))
        .filter(|&saving| saving > 0)
        .counts()
}

fn part1_naive(data: &str) -> usize {
    cheat_counts_naive(data)
        .into_iter()
        .filter(|&(saving, c)| saving >= 100)
        .map(|(_, count)| count)
        .sum()
}

// Return a vector where
// - the index of each item is a number of steps from the position 'pos',
// - the item at this index is the set of points which can be reached from 'pos' with this number of steps.
fn points_at_distance(pos: &PointSet, track: &PointSet) -> Vec<PointSet> {
    successors(Some((pos.clone(), pos.clone())), |(current, visited)| {
        let new = &current.shake() & &track;

        Some((&new & &!visited, &new | visited))
    })
    .map(|(new, _)| new)
    .take_while(|new| !new.is_empty())
    .collect()
}

fn cheat_counts(data: &str) -> HashMap<usize, usize> {
    let map = parse(data);
    let base_time = shortest_path_length(&map);

    // Index is distance from start
    let points_from_start = points_at_distance(&map.start, &map.track);

    // Map each point on the track to its distance to the end
    let distance_to_end: HashMap<Point, usize> = points_at_distance(&map.end, &map.track)
        .iter()
        .enumerate()
        .filter(|(distance, _)|
            // Do not look at points which are too far away from the best path.
            // Note that we would need one more step from the neighboring wall and then another one
            // before that to get this wall, so we can only save something if we are three units
            // closer to the start then the end.
            *distance < base_time - 3
        )
        .flat_map(|(distance, points)| {
            println!("distance={}", distance);
            points.points().into_iter().map(move |p| (p, distance))
        })
        .collect();

    points_from_start
        .iter()
        .enumerate()
        .filter(|(distance, _)|
            // Do not look at points which are too far away from the best path.
            // Note that we would need one more step to the neighboring wall and then another one to
            // get to the next neighbor, so we can only save something if we are three units closer
            // to the start then the end.
            *distance <= base_time - 3
        )
        .flat_map(|(distance, p)| {
            (p.shake() & &map.walls)
                .points()
                .iter()
                .map(|wall| {
                    (
                        distance + 1,
                        PointSet::from_point(&map.walls.dimensions, wall),
                    )
                })
                .collect::<Vec<_>>()
        })
        .flat_map(|(start_to_wall, wall)| {
            (&wall.shake() & &map.track)
                .points()
                .into_iter()
                .map(|neighbor| distance_to_end.get(&neighbor))
                .filter(|optional_distance_to_end| optional_distance_to_end.is_some())
                .map(move |neighbor_to_end| start_to_wall + 1 + neighbor_to_end.unwrap())
        })
        .map(|time| base_time as i64 - time as i64)
        .filter(|&saving| saving > 0)
        .map(|saving| saving as usize)
        .counts()
}

fn part1(data: &str) -> usize {
    cheat_counts(data)
        .into_iter()
        .filter(|&(saving, c)| saving >= 100)
        .map(|(_, count)| count)
        .sum()
}

fn part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_shortest_path_length() {
        assert_eq!(84, shortest_path_length(&parse(TEST_INPUT)))
    }

    #[test]
    fn test_shortest_path_length_with_cheat() {
        assert_eq!(
            72,
            shortest_path_length_with_cheat(
                &parse(TEST_INPUT),
                &Point { x: 8, y: 1 },
                Direction::Right
            )
        );
        assert_eq!(
            64,
            shortest_path_length_with_cheat(
                &parse(TEST_INPUT),
                &Point { x: 10, y: 7 },
                Direction::Right
            )
        );
        assert_eq!(
            84 - 38,
            shortest_path_length_with_cheat(
                &parse(TEST_INPUT),
                &Point { x: 8, y: 8 },
                Direction::Down
            )
        );
        assert_eq!(
            84 - 64,
            shortest_path_length_with_cheat(
                &parse(TEST_INPUT),
                &Point { x: 6, y: 7 },
                Direction::Left
            )
        );
    }

    #[test]
    fn test_cheat_counts() {
        let expected_cheat_counts = HashMap::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]);

        assert_eq!(
            expected_cheat_counts,
            cheat_counts_naive(TEST_INPUT)
        );

        assert_eq!(
            expected_cheat_counts,
            cheat_counts(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        // actually a boring test - the test map does not have cheats with savings >= 100
        assert_eq!(0, part1_naive(TEST_INPUT));
        assert_eq!(0, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        //assert_eq!(42, part2(TEST_INPUT));
    }
}
