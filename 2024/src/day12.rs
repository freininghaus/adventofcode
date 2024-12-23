use std::collections::HashSet;

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
    0
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
        assert_eq!(42, part2(TEST_INPUT));
    }
}
