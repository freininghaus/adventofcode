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

fn part1_impl(width: usize, height: usize, seconds: i32, data: &str) -> usize {
    let x_middle = (width / 2) as i32;
    let y_middle = (height / 2) as i32;

    let robots_in_quadrant = parse(data).into_iter()
        .map(
            |((x0, y0), (vx, vy))|
                ((x0 + seconds * vx) % (width as i32), (y0 + seconds * vy) % (height as i32))
        )
        .map(
            |(x, y)|
                // Note that a % b always has the same sign as a.
                (
                    if x < 0 { x + width as i32 } else { x },
                    if y < 0 { y + height as i32 } else { y }
                )
        )
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

fn part2(data: &str) -> usize {
    0
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
        assert_eq!(42, part2(TEST_INPUT));
    }
}
