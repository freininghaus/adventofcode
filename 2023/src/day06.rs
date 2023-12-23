use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace1, u64};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;

fn input() -> String {
    std::fs::read_to_string("input/day06.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> usize {
    parse_races(lines).iter()
        .map(Race::ways_to_win)
        .fold(1, |a, b| a * b)
}

fn part2(lines: &str) -> usize {
    // We could write a separate parsing function, but re-using and adjusting the result of part 1
    // seems quicker to implement.
    // TODO: it would be great if we could avoid the allocation of the temporary Vecs.
    let (times, distances): (Vec<String>, Vec<String>) = parse_races(lines).iter()
        .map(|Race{ time, distance }| (time.to_string(), distance.to_string()))
        .unzip();

    let (total_time, total_distance): (u64, u64) = [times, distances].into_iter()
        .map(|numbers| numbers.concat().parse().unwrap())
        .next_tuple()
        .unwrap();

    Race{ time: total_time, distance: total_distance }.ways_to_win()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn ways_to_win(&self) -> usize {
        // This could be done faster, but just iterating over all possibilities is fast enough
        (1..self.time)
            .map(|button_hold_time| button_hold_time * (self.time - button_hold_time))
            .filter(|&distance| distance > self.distance)
            .count()
    }
}

fn parse_races(lines: &str) -> Vec<Race> {
    map(tuple((
        tag("Time:"),
        multispace1::<&str, ()>,
        separated_list1(multispace1, u64),
        line_ending,
        tag("Distance:"),
        multispace1,
        separated_list1(multispace1, u64)
    )),
        |(_, _, times, _, _, _, distances)|
            times.into_iter().zip(distances)
                .map(|(time, distance)| Race{ time, distance })
                .collect()
    )(lines)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(288, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503, part2(TEST_INPUT));
    }
}