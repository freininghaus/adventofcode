#[path = "../utils/pointset.rs"]
mod utils;

use crate::utils::{parse_point_sets, point_sets_map, Direction, Point};
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

fn gps_coordinates_sum(boxes: &PointSet) -> usize {
    (0..boxes.dimensions.height)
        .flat_map(|y|
            (0..boxes.dimensions.width)
                .map(move |x| Point { x, y })
        )
        .filter(|p| boxes.contains(&p))
        .map(|Point { x, y }| 100 * y + x)
        .sum()
}

fn part1(data: &str) -> usize {
    let (
        Map { walls, mut boxes, mut robot},
        instructions
    ) = parse(data);

    for direction in instructions {
        try_move(&mut robot, direction, &mut boxes, &walls);
    }

    gps_coordinates_sum(&boxes)
}

// The same as try_move, but for the wider boxes of part 2.
// Note that 'boxes' contains only the left part of each box.
fn try_move_wide(
    object: &mut PointSet,
    direction: Direction,
    all_boxes: &mut PointSet,
    walls: &PointSet,
) -> bool {
    let point = object.points().into_iter().next().unwrap();
    let x = point.x;
    let y = point.y;

    let destination = object.shift(direction);

    // Find out if we try to move the robot or a box
    let is_box = !(&*object & &*all_boxes).is_empty();

    // If we try to move a box, we also have to consider its right part.
    let object_wide = if is_box {
        &*object | &object.shift(Direction::Right)
    } else {
        object.clone()
    };

    let destination_wide = if is_box {
        &destination | &destination.shift(Direction::Right)
    } else {
        destination.clone()
    };

    // Determine the new point(s) which we want to occupy with the object
    let target = &destination_wide & &!&object_wide;

    if !(&target & walls).is_empty() {
        // cannot move into a wall
        return false;
    }

    let move_box = |the_box: &PointSet, all_boxes: &mut PointSet| {
        *all_boxes &= &!the_box;
        *all_boxes |= &the_box.shift(direction);
    };

    if (&target & &(&*all_boxes | &all_boxes.shift(Direction::Right))).is_empty() {
        // 'destination'' is free
        if is_box {
            move_box(&object, all_boxes);
        }

        *object = destination;
        return true;
    }

    // 'target' overlaps with at least one box, but we can try to move it.

    // We might have to try to move multiple boxes which are obstacles.
    // If we can move the first one successfully, but fail at moving the second,
    // we have to revert the changes which would have been made by the first move.
    // Therefore, we remember the original box position.
    let all_boxes_backup = all_boxes.clone();

    let boxes_to_move = (&target | &target.shift(Direction::Left)) & &*all_boxes;

    for box_to_move in boxes_to_move.points() {
        let mut box_to_move = PointSet::from_point(&all_boxes.dimensions, &box_to_move);
        if !try_move_wide(&mut box_to_move, direction, all_boxes, walls) {
            // Undo any moves of boxes at target points that were checked earlier
            *all_boxes = all_boxes_backup;
            return false;
        }
    }

    // the box could be moved
    if is_box {
        move_box(&object, all_boxes);
    }

    *object = destination;

    true
}

fn part2(data: &str) -> usize {
    let data= data.chars()
        .map(|c| match c {
            '#' => "##",
            '.' => "..",
            '@' => "@.",
            '\n' => "\n",

            // we only track the left part of the wider box for simplicity
            'O' => "O.",

            // do not touch move instructions
            '>' => ">",
            '<' => "<",
            '^' => "^",
            'v' => "v",

            _ => panic!("Unexpected character in input: '{}'", c)
        })
        .join("");

    let (
        Map { walls, mut boxes, mut robot},
        instructions
    ) = parse(&data);

    for direction in instructions {
        try_move_wide(&mut robot, direction, &mut boxes, &walls);
    }

    gps_coordinates_sum(&boxes)
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

    const TEST_INPUT_SMALL_PART2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn debug_part2() {
        part2(TEST_INPUT_SMALL_PART2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(9021, part2(TEST_INPUT));
    }
}
