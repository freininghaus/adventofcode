use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day08.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn dimensions(data: &str) -> (i32, i32) {
    (data.lines().count() as i32, data.lines().next().unwrap().len() as i32)
}

fn parse(data: &str) -> HashMap<char, Vec<(i32, i32)>> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (c, (x as i32, y as i32)))
        })
        .sorted()
        .chunk_by(|(c, _)| *c)
        .into_iter()
        .map(|(c, items)|
            (c,
             items.into_iter()
                 .map(|(_c, point)| point)
                 .collect())
        )
        .collect()
}

fn part1(data: &str) -> usize {
    let (width, height) = dimensions(data);
    let antennas = parse(data);

    antennas.iter()
        .flat_map(|(_c, points)| {
            (0..points.len())
                .flat_map(move |i| (i+1..points.len())
                    .flat_map(move |j| {
                        let (x1, y1) = points[i];
                        let (x2, y2) = points[j];

                        vec![(2*x1 - x2, 2*y1 - y2),
                             (2*x2 - x1, 2*y2 - y1)]
                    }))
        })
        .filter(|(x, y)| 0 <= *x && *x < width && 0 <= *y && *y < height)
        .collect::<HashSet<_>>()
        .len()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn part2(data: &str) -> usize {
    let (width, height) = dimensions(data);
    let antennas = parse(data);

    antennas.iter()
        .flat_map(|(_c, points)| {
            (0..points.len())
                .flat_map(move |i| (i+1..points.len())
                    .flat_map(move |j| {
                        let (x1, y1) = points[i];
                        let (x2, y2) = points[j];

                        // vector pointing from (x1, y1) to (x2, y2)
                        let dx = x2 - x1;
                        let dy = y2 - y1;

                        // normalize vector
                        let g = gcd(dx.abs() as usize, dy.abs() as usize) as i32;
                        let dx = dx / g;
                        let dy = dy / g;

                        // go from each point towards the edge of the map in discrete steps
                        vec![((x2, y2), (dx, dy)), ((x1, y1), (-dx, -dy))].iter()
                            .flat_map(
                                move |((x, y), (dx, dy))|
                                    (0..)
                                        .map(move |c| (x + c * dx, y + c * dy))
                                        .take_while(|(x, y)| 0 <= *x && *x < width && 0 <= *y && *y < height)
                                        .collect::<Vec<_>>()  // TODO: does not compile without collecting
                            )
                            .collect::<Vec<_>>()  // TODO: does not compile without collecting
                    }))
        })
        .filter(|(x, y)| 0 <= *x && *x < width && 0 <= *y && *y < height)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        assert_eq!(14, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(34, part2(TEST_INPUT));
    }
}
