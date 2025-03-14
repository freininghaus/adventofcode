use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::successors;

fn input() -> String {
    std::fs::read_to_string("input/day20.txt").expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let Point { x, y } = *self;

        vec![Point { x, y: y - 1 },
             Point { x, y: y + 1 },
             Point { x: x - 1, y },
             Point { x: x + 1, y }]
    }
}

struct Map {
    walls: HashSet<Point>,
    track: HashSet<Point>,
    start: Point,
    end: Point,
}

fn parse(data: &str) -> Map {
    let points_by_char = data.lines().enumerate()
        .flat_map(|(y, line)|
            line.chars().enumerate()
                .map(move |(x, c)| (c, Point { x: x as i32, y: y as i32 }))
        )
        .into_group_map();

    let start = *points_by_char.get(&'S').unwrap().iter().next().unwrap();
    let end = *points_by_char.get(&'E').unwrap().iter().next().unwrap();
    let walls: HashSet<Point> = points_by_char.get(&'#').unwrap().iter().copied().collect();

    let mut track: HashSet<Point> = points_by_char.get(&'.').unwrap().iter().copied().collect();
    track.insert(start);
    track.insert(end);

    Map {
        walls,
        track,
        start,
        end,
    }
}

fn shortest_path_length(map: &Map) -> usize {
    let start: HashSet<Point> = HashSet::from([map.start]);

    let mut visited = start.clone();

    successors(Some(start), |current| {
        let new: HashSet<Point> = current.iter()
            .flat_map(|p| p.neighbors())
            .into_iter()
            .filter(|p| !map.walls.contains(p))
            .filter(|p| !visited.contains(p))
            .collect();

        if (!new.contains(&map.end)) {
            visited.extend(new.iter());
            Some(new)
        } else {
            None
        }
    })
        .count()
}

// Return a vector where
// - the index of each item is a number of steps from the position 'pos',
// - the item at this index is the set of points which can be reached from 'pos' with this number of steps.
fn points_at_distance(pos: Point, map: &Map) -> Vec<HashSet<Point>> {
    successors(
        Some((HashSet::from([pos]), HashSet::from([pos]))),
        |(current, visited)| {
            let new: HashSet<Point> = current.iter()
                .flat_map(|p| p.neighbors())
                .filter(|p| map.track.contains(p))
                .filter(|p| !visited.contains(p))
                .collect();

            Some((new.clone(), new.union(visited).copied().collect()))
        })
        .map(|(new, _)| new)
        .take_while(|new| !new.is_empty())
        .collect()
}

fn cheat_counts(map: &Map) -> HashMap<usize, usize> {
    let base_time = shortest_path_length(&map);

    // Note that we do not look at points which are too far away from the best path.

    // Index is distance from start
    let points_from_start = points_at_distance(map.start, &map);

    // Map each point on the track to its distance to the end
    let distance_to_end: HashMap<Point, usize> = points_at_distance(map.end, &map)
        .into_iter().enumerate()
        .filter(|(distance, _)| *distance <= base_time - 2)
        .flat_map(|(distance, points)|
            points.into_iter().map(move |p| (p, distance))
        )
        .collect();

    // Create a reference that can be moved into a closure
    let distance_to_end_ref = &distance_to_end;

    // Calculate the distance of each wall cell from the start, assuming that it is permitted to
    // take the last step from a track cell to the wall cell.
    let wall_distance_from_start: HashMap<Point, usize> = points_from_start.into_iter()
        .enumerate()
        // Note that we reverse the iterator because the items that are collected later into the HashMap
        // remain.
        .rev()
        .filter(|(distance, _)| *distance <= base_time - 2)
        .flat_map(|(distance, points)| {
            points.into_iter().flat_map(move |p|
                p.neighbors().into_iter()
                    .filter(|n| map.walls.contains(n))
                    .map(move |n| (n, distance + 1))
            )
        })
        .collect();

    wall_distance_from_start.into_iter()
        .flat_map(|(wall, distance_from_start)|
            wall.neighbors().into_iter()
                .flat_map(move |n|
                    distance_to_end_ref.get(&n)
                        .map(|to_end| distance_from_start + 1 + *to_end))
        )
        .map(|time| base_time as i64 - time as i64)
        .filter(|&saving| saving > 0)
        .map(|saving| saving as usize)
        .counts()
}


fn part1(data: &str) -> usize {
    let map = parse(data);
    cheat_counts(&map).into_iter()
        .filter(|&(saving, c)| saving >= 100)
        .map(|(_, c)| c)
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

        let map = parse(TEST_INPUT);

        assert_eq!(
            expected_cheat_counts,
            cheat_counts(&map)
        );
    }

    #[test]
    fn test_part1() {
        // actually a boring test - the test map does not have cheats with savings >= 100
        assert_eq!(0, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        //assert_eq!(42, part2(TEST_INPUT));
    }
}
