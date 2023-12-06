use std::collections::HashSet;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day03.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    let symbol_positions = symbol_positions(lines);
    let numbers = parse_numbers(lines);

    numbers.iter()
        .filter(|number| number.is_part_number(&symbol_positions))
        .map(|number| number.value)
        .sum()
}

fn part2(lines: &str) -> u32 {
    let numbers = parse_numbers(lines);
    let numbers = &numbers; // See https://stackoverflow.com/questions/58459643/is-there-a-way-to-have-a-rust-closure-that-moves-only-some-variables-into-it

    parse_symbols(lines).iter()
        .enumerate()
        .flat_map(move |(y, row)| row.iter()
            .enumerate()
            .filter(|(_x, &symbol)| symbol == Some('*'))
            .map(move |(x, _)| numbers.iter()
                .filter(move |number| is_adjacent(number, &(x, y)))
                .collect::<Vec<_>>()
            )
            .filter(|adjacent_numbers| adjacent_numbers.len() == 2)
            .map(|adjacent_numbers| adjacent_numbers[0].value * adjacent_numbers[1].value)
        )
        .sum()
}

#[derive(Debug)]
struct Number {
    value: u32,
    y: usize,
    x_min: usize,
    x_max: usize
}

impl Number {
    fn is_part_number(&self, symbols: &HashSet<(usize, usize)>) -> bool {
        let Self { value: _value, y, x_min, x_max } = *self;
        (x_min..=x_max).any(|x| (-1..=1).any(
            |dx| (-1..=1).any(
                |dy| symbols.contains(&(
                    x.saturating_add_signed(dx),
                    y.saturating_add_signed(dy)
                ))
            )
        ))
    }
}

fn is_adjacent(number: &Number, &(symbol_x, symbol_y): &(usize, usize)) -> bool {
    let Number { value: _value, y: number_y, x_min, x_max } = *number;
    (x_min..=x_max).any(|number_x| (-1..=1).any(
        |dx| (-1..=1).any(
            |dy| symbol_x == number_x.saturating_add_signed(dx) &&
                symbol_y == number_y.saturating_add_signed(dy)
            ))
        )
}

fn parse_symbols(lines: &str) -> Vec<Vec<Option<char>>> {
    lines.lines()
        .map(|row|
            row.chars()
                .map(|c| match c {
                    '.' => None,
                    '0'..='9' => None,
                    _ => Some(c)
                })
                .collect()
        )
        .collect()
}

fn symbol_positions(lines: &str) -> HashSet<(usize, usize)> {
    parse_symbols(lines).iter()
        .enumerate()
        .flat_map(move |(y, row)| row.iter()
            .enumerate()
            .filter(|(_x, cell)| cell.is_some())
            .map(move |(x, _)| (x, y)))
        .collect()
}

fn parse_numbers(lines: &str) -> Vec<Number> {
    // TODO: group by (y, c.is_digit(10)), then filter for is_digit=true, then return this to flat_map
    // (this might save the collect() into the temporary Vec)
    lines.lines()
        .enumerate()
        .flat_map(|(y, row)| row.chars()
            .enumerate()
            .group_by(|(_x, c)| c.is_digit(10))
            .into_iter()
            .filter(|(is_number, _)| *is_number)
            .map(|(_, group)| (y, group.unzip()))
            .into_iter().collect::<Vec<(usize, (Vec<usize>, String))>>()
        )
        .map(|(y, (xs, number_as_string))|
            Number {
                y: y,
                x_min: *xs.first().unwrap(),
                x_max: *xs.last().unwrap(),
                value: number_as_string.parse().unwrap()
            })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1(TEST_INPUT_1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(467835, part2(TEST_INPUT_1));
    }
}