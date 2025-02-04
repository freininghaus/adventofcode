#[path = "../utils/pointset.rs"]
mod utils;

use crate::utils::{parse_point_sets, Direction, Point};
use itertools::Itertools;
use std::collections::HashSet;
use utils::PointSet;

fn input() -> String {
    std::fs::read_to_string("input/day15.txt").expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

struct Map {
    walls: PointSet,
    boxes: PointSet,
    robot: PointSet,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut point_sets = parse_point_sets(input);

        if point_sets.keys().map(|c| *c).collect::<HashSet<_>>()
            != HashSet::from_iter("#O@.".chars())
        {
            panic!("Unexpected characters in input map");
        }

        Map {
            walls: point_sets.remove(&'#').unwrap(),
            boxes: point_sets.remove(&'O').unwrap(),
            robot: point_sets.remove(&'@').unwrap(),
        }
    }
}

fn parse(input: &str) -> (Map, Vec<Direction>) {
    let (map, instructions) = input.split("\n\n").collect_tuple().unwrap();

    (
        Map::parse(map),
        instructions
            .chars()
            .filter(|c| *c != '\n')
            .map(Direction::try_from)
            .map(Result::unwrap)
            .collect(),
    )
}

// Find out whether an object can be moved in the given direction. Movable boxes and walls, which
// cannot be moved, are considered.
//
// If moving is possible, the function
// - updates 'object'
// - updates 'all_boxes', but only concerning boxes which are not equal to 'object', i.e., only
//   the positions of boxes which are obstacles for the object are updated
// - returns 'true'.
//
// If moving is not possible, the function returns 'false' and leaves the object and all_boxes
// unchanged.
fn try_move(
    object: &mut PointSet,
    direction: Direction,
    all_boxes: &mut PointSet,
    walls: &PointSet,
) -> bool {
    let destination = object.shift(direction);

    if !(&destination & walls).is_empty() {
        // cannot move into a wall
        return false;
    }

    if (&destination & &*all_boxes).is_empty() {
        // 'destination'' is free
        *object = destination;
        return true;
    }

    // there is a box at 'destination', but we can try to move it
    if try_move(&mut destination.clone(), direction, all_boxes, walls) {
        // the box could be moved
        *all_boxes &= &!&destination;
        *all_boxes |= &destination.shift(direction);
        *object = destination;

        return true;
    }

    false
}

fn part1(data: &str) -> usize {
    let (
        Map { walls, mut boxes, mut robot},
        instructions
    ) = parse(data);

    for direction in instructions {
        try_move(&mut robot, direction, &mut boxes, &walls);
    }

    (0..walls.dimensions.height)
        .flat_map(|y|
            (0..walls.dimensions.width)
                .map(move |x| Point { x, y })
        )
        .filter(|p| boxes.contains(&p))
        .map(|Point { x, y }| 100 * y + x)
        .sum()
}

fn part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::point_sets_map;

    #[test]
    fn test_move_right() {
        let Map {
            walls,
            boxes,
            robot,
        } = Map::parse(
            "##########
#@.O.O.O.#
##########",
        );

        // Moving up, down, left does not work
        let mut mut_boxes = boxes.clone();
        let mut mut_robot = robot.clone();

        for direction in [Direction::Up, Direction::Down, Direction::Left] {
            assert!(!try_move(&mut mut_robot, direction, &mut mut_boxes, &walls));
            assert_eq!(boxes, mut_boxes);
            assert_eq!(robot, mut_robot);
        }

        // Moving right a few times works
        assert!(try_move(&mut mut_robot, Direction::Right, &mut mut_boxes, &walls));
        let map = "##########
# @O O O #
##########";
        assert_eq!(map, point_sets_map(&vec![('#', &walls), ('@', &mut_robot), ('O', &mut_boxes)]));

        assert!(try_move(&mut mut_robot, Direction::Right, &mut mut_boxes, &walls));
        let map = "##########
#  @OO O #
##########";
        assert_eq!(map, point_sets_map(&vec![('#', &walls), ('@', &mut_robot), ('O', &mut_boxes)]));

        assert!(try_move(&mut mut_robot, Direction::Right, &mut mut_boxes, &walls));
        let map = "##########
#   @OOO #
##########";
        assert_eq!(map, point_sets_map(&vec![('#', &walls), ('@', &mut_robot), ('O', &mut_boxes)]));

        assert!(try_move(&mut mut_robot, Direction::Right, &mut mut_boxes, &walls));
        let map = "##########
#    @OOO#
##########";
        assert_eq!(map, point_sets_map(&vec![('#', &walls), ('@', &mut_robot), ('O', &mut_boxes)]));

        // cannot move futher once the boxes touch the right wall
        assert!(!try_move(&mut mut_robot, Direction::Right, &mut mut_boxes, &walls));
        assert_eq!(map, point_sets_map(&vec![('#', &walls), ('@', &mut_robot), ('O', &mut_boxes)]));
    }

    const TEST_INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        assert_eq!(2028, part1(TEST_INPUT_SMALL));
        assert_eq!(10092, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        //assert_eq!(42, part2(TEST_INPUT));
    }
}
