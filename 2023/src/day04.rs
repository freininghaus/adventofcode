use std::cmp::min;
use std::collections::{HashMap, HashSet};
use nom::bytes::complete::tag;
use nom::character;
use nom::multi::separated_list1;
use nom::sequence::tuple;

fn input() -> String {
    std::fs::read_to_string("input/day04.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    lines.lines()
        .map(parse_card)
        .map(|Card{ id: _, winning_numbers, numbers_we_have}|
            numbers_we_have.iter()
                .filter(|n| winning_numbers.contains(n))
                .count())
        .filter(|&count| count > 0)
        .map(|count| 1 << (count - 1))
        .sum()
}

fn part2(lines: &str) -> usize {
    let cards: HashMap<usize, Card> = lines.lines()
        .map(parse_card)
        .map(|card| (card.id, card))
        .collect();

    let won_cards = cards.iter()
        .map(|(_id, Card{id, winning_numbers, numbers_we_have})|
            (id, numbers_we_have.iter()
                .filter(|n| winning_numbers.contains(n))
                .count())
            )
        .collect::<HashMap<_, _>>();

    let min_card_id = *cards.keys().min().unwrap();
    let max_card_id = *cards.keys().max().unwrap();
    let reversed_card_ids = (min_card_id..=max_card_id).rev();

    let total_cards_per_original_card = reversed_card_ids.into_iter()
        .fold(
            HashMap::new(),
            |mut known, id| {
                // Calculate the directly won cards
                let direct_wins =
                    id + 1..=id + min(
                        max_card_id,
                        *won_cards.get(&id).unwrap())   ;

                // Calculate the total number of cards we have if we initially have one with the current id
                let total = 1 + direct_wins.map(|i| known.get(&i).unwrap()).sum::<usize>();

                known.insert(id, total);
                known
            }
        );

    total_cards_per_original_card.values().sum()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<u32>,
    numbers_we_have: Vec<u32>
}

fn parse_card(line: &str) -> Card {
    let numbers = || separated_list1(
        character::complete::space1,
        character::complete::u32::<&str, ()>
    );

    let mut parser = tuple((
        tag("Card"),
        character::complete::space1,
        character::complete::u32::<&str, ()>,
        character::complete::char(':'),
        character::complete::space1,
        numbers(),
        character::complete::space1,
        character::complete::char('|'),
        character::complete::space1,
        numbers()
    ));

    let (_, _, id, _, _, winning_numbers, _, _, _, numbers_we_have) = parser(line).unwrap().1;

    Card {
        id: id as usize,
        winning_numbers: winning_numbers.into_iter().collect(),
        numbers_we_have
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }

    #[test]
    fn testi() {
        let numbers = || separated_list1(
            character::complete::space1,
            character::complete::u32::<&str, ()>
        );

        let mut parser = tuple((
            tag("Card "),
            character::complete::u32::<&str, ()>,
            character::complete::char(':'),
            character::complete::space1,
            numbers(),
            character::complete::space1,
            character::complete::char('|'),
            character::complete::space1,
            numbers()
        ));

        let result = parser(TEST_INPUT.lines().next().unwrap());

        println!("{:?}", result);
    }
}