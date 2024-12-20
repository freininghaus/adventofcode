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

fn neighbors((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }

    result.push((x + 1, y));
    result.push((x, y + 1));

    result
}

fn parse(data: &str) -> Vec<(char, HashSet<(usize, usize)>)> {
    let mut map: Vec<Vec<Option<char>>> = data.lines()
        .map(|line| line.trim().chars().map(|c| Some(c)).collect())
        .collect();

    let mut result: Vec<(char, HashSet<(usize, usize)>)> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let Some(c) = map[y][x].take() else {
                continue;
            };

            let mut plot: HashSet<(usize, usize)> = HashSet::new();
            let mut to_do = vec![(x, y)];

            // Note that process() cannot extend to_do itself because capturing to_do would be a
            // second mutable borrow in addition to the call to to_do.pop below.
            let mut process = |(x, y): (usize, usize)| -> Vec<(usize, usize)> {
                // We add 1 to the x and y coordinates such that we can re-use the neighbors()
                // function for determining the perimeter. With 0-based indexing, the cell at (0, 0
                // would have a neighbor at, e.g., (-1, 0), which is outside the range of usize.
                // We could use signed integers instead, but then we would need type casts elsewhere.
                plot.insert((x + 1, y + 1));
                neighbors((x, y))
                    .into_iter()
                    .filter(
                        |&(x, y)| {
                            let Some(row) = map.get_mut(y) else {
                                return false;
                            };

                            let Some(cell) = row.get_mut(x) else {
                                return false;
                            };

                            cell.take_if(|letter| *letter == c).is_some()
                        }
                    )
                    .collect()
            };

            while let Some((x, y)) = to_do.pop() {
                let new_cells_in_plot = process((x, y));
                to_do.extend(new_cells_in_plot);
            }

            result.push((c, plot));
        }
    }

    result
}

fn part1(data: &str) -> usize {
    let plots = parse(data);

    plots.iter()
        .map(|(_c, plot)| {
            let area = plot.len();
            let perimeter: usize = plot.iter()
                .map(|(x, y)|
                    neighbors((*x, *y)).into_iter()
                        .filter(|neighbor| !plot.contains(neighbor))
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
