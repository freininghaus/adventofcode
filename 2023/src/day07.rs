use itertools::{iproduct, Itertools};
use nom::ParseTo;

fn input() -> String {
    std::fs::read_to_string("input/day07.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str) -> u32 {
    parse_part1(lines).iter()
        .sorted()
        .map(|(_hand, bid)| bid)
        .zip(1..)
        .map(|(bid, rank)| bid * rank)
        .sum()
}

fn part2(lines: &str) -> u32 {
    parse_part2(lines).iter()
        .sorted()
        .map(|(_hand, bid)| bid)
        .zip(1..)
        .map(|(bid, rank)| bid * rank)
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl HandType {
    fn from(hand: &[u8; 5]) -> Self {
        let mut card_counts = hand.iter()
            .sorted()
            .group_by(|card| **card)
            .into_iter()
            .map(|(_card_label, cards)| cards.count())
            .sorted()
            .rev();

        use HandType::*;

        match card_counts.next().unwrap() {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => match card_counts.next().unwrap() {
                2 => FullHouse,
                _ => ThreeOfAKind
            },
            2 => match card_counts.next().unwrap() {
                2 => TwoPair,
                _ => OnePair
            },
            _ => HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    card_value: u32
}

impl Hand {
    fn new(cards: &[u8]) -> Self {
        let cards: [u8; 5] = cards.try_into().expect("A hand consists of five cards!");

        Hand {
            hand_type: HandType::from(&cards),
            card_value: cards.into_iter()
                .map(|card| card_value(card))
                .fold(0, |total, value| total << 4 | value as u32)
        }
    }

    fn new_with_jokers(cards: &[u8]) -> Self {
        let cards: [u8; 5] = cards.try_into().expect("A hand consists of five cards!");

        // 'J' is now the weakest card
        let card_value = cards.iter()
            .map(|card|
                if *card == b'J' {
                    0
                } else {
                    card_value(*card)
                })
            .fold(0, |total, value| total << 4 | value as u32);

        let (orig0, orig1, orig2, orig3, orig4) = cards.iter().next_tuple().unwrap();

        // TODO: do not confuse with izip!
        iproduct!(
            possible_substitutions(*orig0),
            possible_substitutions(*orig1),
            possible_substitutions(*orig2),
            possible_substitutions(*orig3),
            possible_substitutions(*orig4)
        )
            .map(|(c0, c1, c2, c3, c4)| {
                let hand_type = HandType::from(&[c0, c1, c2, c3, c4]);
                Hand { hand_type, card_value }})
            .max()
            .unwrap()
    }
}

fn possible_substitutions(card: u8) -> Vec<u8> {
    if card != b'J' {
        vec![card]
    } else {
        vec![b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A']
    }
}

fn card_value(card: u8) -> u8 {
    match card {
        b'2'..=b'9' => card - b'0',
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("invalid card")
    }
}

fn parse_part1(lines: &str) -> Vec<(Hand, u32)> {
    lines.lines()
        .map(|line| line.split_whitespace().map(|s| s.as_bytes()).next_tuple().unwrap())
        .map(|(hand, bid)| (
            Hand::new(hand),
            bid.parse_to().expect("Cannot parse bid as u32"))
        )
        .collect()
}

fn parse_part2(lines: &str) -> Vec<(Hand, u32)> {
    lines.lines()
        .map(|line| line.split_whitespace().map(|s| s.as_bytes()).next_tuple().unwrap())
        .map(|(hand, bid)| (
            Hand::new_with_jokers(hand),
            bid.parse_to().expect("Cannot parse bid as u32"))
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::HandType::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_hand_type() {
        assert_eq!(HandType::from(b"32T3K"), OnePair);
        assert_eq!(HandType::from(b"T55J5"), ThreeOfAKind);
        assert_eq!(HandType::from(b"KK677"), TwoPair);
        assert_eq!(HandType::from(b"KTJJT"), TwoPair);
        assert_eq!(HandType::from(b"QQQJA"), ThreeOfAKind);
    }

    #[test]
    fn test_hand_type_with_joker() {
        assert_eq!(Hand::new_with_jokers(b"32T3K").hand_type, OnePair);
        assert_eq!(Hand::new_with_jokers(b"T55J5").hand_type, FourOfAKind);
        assert_eq!(Hand::new_with_jokers(b"KK677").hand_type, TwoPair);
        assert_eq!(Hand::new_with_jokers(b"KTJJT").hand_type, FourOfAKind);
        assert_eq!(Hand::new_with_jokers(b"QQQJA").hand_type, FourOfAKind);
    }

    #[test]
    fn test_part1() {
        assert_eq!(6440, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5905, part2(TEST_INPUT));
    }
}