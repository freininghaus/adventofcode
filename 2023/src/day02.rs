use std::cmp::max;
use std::iter::Sum;
use nom::{
    bytes::complete::tag,
    character,
    sequence::tuple,
    branch::alt,
    combinator::map,
    IResult,
    multi::separated_list1,
};

use std::ops::Add;

fn input() -> String {
    std::fs::read_to_string("input/day02.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    max_cube_counts_per_game(lines).iter()
        .filter(
            |(_id, CubeCounts { red, green, blue })|
            *red <= 12 && *green <= 13 && *blue <= 14)
        .map(|(id, _)| id)
        .sum()
}

fn part2(lines: &str) -> u32 {
    max_cube_counts_per_game(lines).into_iter()
        .map(|(_id, CubeCounts { red, green, blue })| red * green * blue)
        .sum()
}

fn max_cube_counts_per_game(lines: &str) -> Vec<(u32, CubeCounts)> {
    lines.lines()
        .map(parse_line)
        .map(|Game { id, cube_counts }| (
            id,
            cube_counts.into_iter().fold(
                CubeCounts::zero(),
                |CubeCounts { red: red_max, green: green_max, blue: blue_max },
                 CubeCounts { red, green, blue }|
                    CubeCounts {
                        red: max(red, red_max),
                        green: max(green, green_max),
                        blue: max(blue, blue_max),
                    },
            )))
        .collect()
}

#[derive(Debug, PartialEq)]
struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32
}

impl CubeCounts {
    fn zero() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }

    fn red(value: u32) -> Self {
        Self { red: value, green: 0, blue: 0 }
    }

    fn green(value: u32) -> Self {
        Self { red: 0, green: value, blue: 0 }
    }

    fn blue(value: u32) -> Self {
        Self { red: 0, green: 0, blue: value }
    }
}

impl Add for CubeCounts {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue
        }
    }
}

impl Sum for CubeCounts {
    fn sum<I>(iter: I) -> Self
        where I: Iterator<Item = Self> {
        iter.fold(CubeCounts::zero(), |acc, counts| acc + counts)
    }

}

fn parse_color<'a, F>(color: &'static str, factory: F) -> impl FnMut(&'a str) -> IResult<&'a str, CubeCounts, ()>
    where F: Fn(u32) -> CubeCounts {
    map(tuple((
        character::complete::char(' '),
        character::complete::u32::<&str, ()>,
        character::complete::char(' '),
        tag(color)
    )),
        move |(_, count, _, _)| factory(count)
    )
}

fn parse_cube_counts<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, CubeCounts, ()> {
    map(
        separated_list1(
            character::complete::char(','),
            alt((
                parse_color("red", CubeCounts::red),
                parse_color("green", CubeCounts::green),
                parse_color("blue", CubeCounts::blue)
            ))
        ),
        |counts| counts.into_iter().sum()
    )
}

struct Game {
    id: u32,
    cube_counts: Vec<CubeCounts>
}

fn parse_line(line: &str) -> Game {
    let mut parser = tuple((
        tag("Game "),
        character::complete::u32::<&str, ()>,
        character::complete::char(':'),
        separated_list1(character::complete::char(';'), parse_cube_counts())
    ));

    let (_, id, _, cube_counts) = parser(line).unwrap().1;

    Game { id: id, cube_counts: cube_counts }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";


    #[test]
    fn foo() {
        let parser = character::complete::u32::<&str, ()>;
        let result = parser("42").unwrap().1;
        println!("{:?}", result);

        let mut parser = tuple((
            tag("Game "),
            character::complete::u32::<&str, ()>,
            character::complete::char(':'),
            separated_list1(character::complete::char(';'), parse_cube_counts())
        ));

        let result = parser("Game 42: 1 red, 2 green, 3 blue; 4 blue, 2 green, 6 red").unwrap().1;
        println!("{:?}", result);
    }

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(TEST_INPUT_1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286, part2(TEST_INPUT_1));
    }
}