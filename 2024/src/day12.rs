use std::collections::HashSet;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day12.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn parse(data: &str) -> Vec<(char, HashSet<(i32, i32)>)> {
    let mut map: Vec<Vec<Option<char>>> = data.lines()
        .map(|line| line.trim().chars().map(|c| Some(c)).collect())
        .collect();

    let mut result: Vec<(char, HashSet<(i32, i32)>)> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let Some(c) = map[y][x].take() else {
                continue;
            };

            // We work with signed integer coordinates, even if they are all negative. The benefit
            // is that we do not have to check if a number is positive if we subtract one to
            // determine the coordinates of a potential neighbor.
            let mut region: HashSet<(i32, i32)> = HashSet::new();
            let mut to_do = vec![(x as i32, y as i32)];

            // Note that process() cannot extend to_do itself because capturing to_do would be a
            // second mutable borrow in addition to the call to to_do.pop below.
            let mut process = |(x, y): (i32, i32)| -> Vec<(i32, i32)> {
                // Add (x, y) to the current region, and add all its neighbors with the same letter
                // to the 'to do' list.
                region.insert((x, y));
                neighbors((x, y))
                    .into_iter()
                    .filter(
                        |&(x, y)| {
                            if x < 0 || y < 0 {
                                return false;
                            }

                            let Some(row) = map.get_mut(y as usize) else {
                                return false;
                            };

                            let Some(cell) = row.get_mut(x as usize) else {
                                return false;
                            };

                            cell.take_if(|letter| *letter == c).is_some()
                        }
                    )
                    .collect()
            };

            while let Some((x, y)) = to_do.pop() {
                let new_cells_in_region = process((x, y));
                to_do.extend(new_cells_in_region);
            }

            result.push((c, region));
        }
    }

    result
}

fn part1(data: &str) -> usize {
    let regions = parse(data);

    regions.iter()
        .map(|(_c, region)| {
            let area = region.len();
            let perimeter: usize = region.iter()
                .map(|(x, y)|
                    neighbors((*x, *y)).into_iter()
                        .filter(|neighbor| !region.contains(neighbor))
                        .count()
                )
                .sum();

            area * perimeter
        })
        .sum()
}

fn part2(data: &str) -> usize {
    let regions = parse(data);

    regions.into_iter()
        .map(|(_c, plots)| {
            let area = plots.len();

            let up = (0, -1);
            let down = (0, 1);
            let left = (-1, 0);
            let right = (1, 0);

            let sides =  [up, down, left, right].into_iter()
                .map(|(dx, dy)| {
                    plots.iter()
                        // Find plots which are part of a side in the given direction.
                        .filter(|(x, y)| !plots.contains(&(*x + dx, *y + dy)))
                        .map(
                            // To simplify further processing, we swap x and y in the up/down case.
                            // This ensures that x is the coordinate which is the same for all plots
                            // that belong to a side, and y is the coordinate that grows in steps
                            // of one unit between plots in the side.
                            |(x, y)|
                                if dx == 0 {
                                    (*y, *x)
                                } else {
                                    (*x, *y)
                                }
                        )
                        .sorted()
                        // Use chunk_by to group into points with the same 'x' value
                        .chunk_by(|(x, _y)| *x)
                        .into_iter()
                        .map(
                            |(_x, plots)|
                                plots
                                    .map(|(_x, y)| y)
                                    .collect::<Vec<_>>()
                                    // Use chunk by to group into points whose y values differ by one
                                    .chunk_by(|y1, y2| y1 + 1 == *y2)
                                    .count()
                        )
                        .sum::<usize>()
                })
                .sum::<usize>();

            area * sides
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(1930, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1206, part2(TEST_INPUT));
    }
}
