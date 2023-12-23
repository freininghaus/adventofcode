use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap};
use std::ops::Range;
use itertools::Itertools;
use nom::{character, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;

fn input() -> String {
    std::fs::read_to_string("input/day05.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u64 {
    let almanac = parse_almanac(lines);
    almanac.lowest_location_for_seeds(&almanac.seeds)
}

fn part2(lines: &str) -> u64 {
    let almanac = parse_almanac(lines);
    let seeds: Vec<Range<u64>> = almanac.seeds.iter()
        .tuples()
        .map(|(&start, &count)| (start..start + count))
        .collect();
    almanac.lowest_location_for_seed_ranges(seeds)
}

#[derive(Debug)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64
}

impl MapRange {
    fn mapped_range(&self, range: &Range<u64>) -> Option<Range<u64>> {
        if self.contains(&range) {
            Some(range.start - self.source_start + self.destination_start
                ..range.end - self.source_start + self.destination_start)
        } else {
            None
        }
    }

    fn contains(&self, range: &Range<u64>) -> bool {
        // only works if 'range' is either a subset of self, or disjoint
        range.start >= self.source_start && range.start <= self.source_start + self.length
    }

    fn split_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let boundaries: BTreeSet<u64> = [
            range.start,
            max(range.start, min(self.source_start, range.end)),
            min(range.end, max(self.source_start + self.length, range.start)),
            range.end
        ].into_iter().collect();

        boundaries.iter().zip(boundaries.iter().skip(1))
            .map(|(&start, &end)| (start..end))
            .collect()
    }
}

#[derive(Debug)]
struct Map {
    map_ranges: Vec<MapRange>
}

impl Map {
    fn mapped_number(&self, n: u64) -> u64 {
        self.map_ranges.iter()
            .flat_map(|MapRange {
                destination_start,
                source_start,
                length
            }| if *source_start <= n && source_start + length >= n {
                Some(n - source_start + destination_start)
            } else {
                None
            })
            .next()
            .unwrap_or(n)
    }

    fn mapped_ranges(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let ranges = self.map_ranges.iter()
            .fold(
                ranges,
                |ranges, map_range|
                    ranges.into_iter()
                        .flat_map(|range| map_range.split_range(range))
                        .collect()
        );

        ranges.into_iter()
            .map(
                |range|
                    self.map_ranges.iter()
                        .flat_map(|map_range| map_range.mapped_range(&range))
                        .next()
                        .unwrap_or(range))
            .collect()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, (String, Map)>
}

impl Almanac {
    fn lowest_location_for_seeds(&self, seeds: &Vec<u64>) -> u64 {
        seeds.iter().map(|seed| {
            let mut category = "seed";
            let mut number = *seed;

            while category != "location" {
                let (new_category, map) = self.maps.get(category).unwrap();
                category = new_category;
                number = map.mapped_number(number);
            }

            number
        }).min().unwrap()
    }

    fn lowest_location_for_seed_ranges(&self, seed_ranges: Vec<Range<u64>>) -> u64 {
        let mut seed_ranges = seed_ranges;
        let mut category = "seed";

        while category != "location" {
            let (new_category, map) = self.maps.get(category).unwrap();
            category = new_category;
            seed_ranges = map.mapped_ranges(seed_ranges);
        }

        seed_ranges.into_iter()
            .map(|Range{ start, .. }| start)
            .min()
            .unwrap()
    }
}

fn parse_range<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MapRange, ()>
{
    map(tuple((
        character::complete::u64::<&str, ()>,
        character::complete::char(' '),
        character::complete::u64::<&str, ()>,
        character::complete::char(' '),
        character::complete::u64::<&str, ()>
    )),
        move
            |(destination_start, _, source_start, _, length)|
            MapRange { destination_start, source_start, length }
    )
}

fn parse_map<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (String, String, Map), ()>
{
    map(tuple((
        character::complete::alpha1,
        tag("-to-"),
        character::complete::alpha1,
        tag(" map:\n"),
        separated_list1(line_ending, parse_range())
    )),
    move
        |(type1, _, type2, _, ranges)|
        (type1.to_string(), type2.to_string(), Map { map_ranges: ranges })
    )
}

fn parse_almanac(lines: &str) -> Almanac {
    map(tuple((
        tag("seeds: "),
        separated_list1(character::complete::space1, character::complete::u64::<&str, ()>),
        character::complete::multispace1,
        separated_list1(tag("\n\n"), parse_map())
    )),
|(_, seeds, _, maps)|
    Almanac {
        seeds,
        maps: maps.into_iter()
            .map(|(from, to, map)| (from, (to, map)))
            .collect()
    })(lines)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(TEST_INPUT));
    }
}