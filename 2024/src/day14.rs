use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;

fn input() -> String {
    std::fs::read_to_string("input/day14.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<((i32, i32), (i32, i32))> {
    let robot_regex = Regex::new(r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();

    data.lines()
        .map(|line| {
            robot_regex.captures(line).unwrap()
                .iter()
                .skip(1)
                .map(|capture| capture.unwrap().as_str().parse().unwrap())
                .chunks(2)
                .into_iter()
                .map(|chunk| chunk.collect_tuple().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn evolve(robot: &((i32, i32), (i32, i32)), width: usize, height: usize, seconds: i32) -> (i32, i32) {
    let ((x0, y0), (vx, vy)) = robot;

    let (x, y) = (
        (x0 + seconds * vx) % (width as i32),
        (y0 + seconds * vy) % (height as i32)
    );

    (
        if x < 0 { x + width as i32 } else { x },
        if y < 0 { y + height as i32 } else { y }
    )
}

fn part1_impl(width: usize, height: usize, seconds: i32, data: &str) -> usize {
    let x_middle = (width / 2) as i32;
    let y_middle = (height / 2) as i32;

    let robots_in_quadrant = parse(data).into_iter()
        .map(|robot| evolve(&robot, width, height, seconds))
        .map(
            |(x, y)|
                ((x - x_middle).signum(), (y - y_middle).signum())
        )
        .filter(|(qx, qy)| *qx != 0 && *qy != 0)
        .counts();

    if robots_in_quadrant.len() < 4 {
        // Not all quadrants have robots, so the product of the counts is zero.
        return 0;
    }

    robots_in_quadrant.values().product()
}

fn part1(data: &str) -> usize {
    part1_impl(101, 103, 100, data)
}

fn advance(width: i32, height: i32, (vx, vy): &(i32, i32), (ref mut x, ref mut y): &mut (i32, i32)) {
    *x = (*x + vx + width) % width;
    *y = (*y + vy + height) % height;
}

fn print_robots(width: usize, height: usize, robot_positions: &HashSet<(i32, i32)>) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", if robot_positions.contains(&((x as i32), (y as i32))) { '*' } else { ' ' });
        }

        println!();
    }
}

fn part2(data: &str) -> usize {
    // This could probably be optimized to run much faster, but the straightforward solution runs
    // in acceptable time.

    let width = 101;
    let height = 103;
    let (mut positions, velocities): (Vec<_>, Vec<_>) = parse(data).into_iter().unzip();

    for second in 0..width * height {
        // Check if many robots are close to another robot
        let all_positions = positions.iter()
            .map(|&pos| pos)
            .collect::<HashSet<_>>();

        let robots_with_neighbors = positions.iter()
            .filter(
                |&&(x, y)|
                    (0..=1)
                        .flat_map(|dx|
                            (0..=1)
                                .map(move |dy| (dx, dy))
                                .filter(|&(dx, dy)| dx != 0 || dy != 0)
                                .map(|(dx, dy)| (x + dx, y + dy))
                        )
                        .any(|(x, y)|
                            all_positions.contains(&(x, y)))
            )
            .count();

        // Terminate if more than half the robots are close to another one
        if robots_with_neighbors > positions.len() / 2 {
            print_robots(width, height, &all_positions);
            return second;
        }

        // Advance all robot positions
        for (v, mut pos) in velocities.iter().zip(&mut positions) {
            advance(width as i32, height as i32, v, &mut pos);
        }
    }

    panic!("Could not find solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(12, part1_impl(11, 7, 100, TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        //assert_eq!(42, part2(TEST_INPUT));
    }
}
