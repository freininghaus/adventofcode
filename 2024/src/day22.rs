use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

fn input() -> String {
    std::fs::read_to_string("input/day22.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<u64> {
    data.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn prune(n: u64) -> u64 {
    n % 16777216
}

fn evolve_secret_number(n: u64) -> u64 {
    let a = prune(n ^ (n  * 64));
    let b = prune(a ^ a / 32);
    let c = prune(b * 2048 ^ b);

    c
}

fn part1(data: &str) -> u64 {
    parse(data).into_iter()
        .map(|mut n| {
            for _ in 0..2000 {
                n = evolve_secret_number(n);
            }

            n
        })
        .sum()
}

// Note that we model the sequence of four price changes with a 32-bit int. Each byte represents an
// item in the price change sequence, so shifting left by 8 bits discards one item and creates some
// space for the next one.
// The byte value for a price change d between -9 and 9 is 100 + d.
//
// The result is a map where the four most recent price changes in this encoded form are the keys,
// and the first number of bananas that corresponds to this price change sequence is the value.
fn bananas_per_price_change_sequence(initial_secret: u64) -> HashMap<u32, u64> {
    let mut secret = initial_secret;
    let mut bananas = secret % 10;
    let mut encoded_recent_changes: u32 = 0;
    let mut result: HashMap<u32, u64> = HashMap::new();

    for step in 0..=2000 {
        if step >= 4 && !result.contains_key(&encoded_recent_changes) {
            result.insert(encoded_recent_changes, bananas);
        }

        secret = evolve_secret_number(secret);
        let new_bananas = secret % 10;

        encoded_recent_changes <<= 8;
        encoded_recent_changes |= (100 + new_bananas - bananas) as u32;

        bananas = new_bananas;
    }

    result
}

fn part2(data: &str) -> u64 {
    *parse(data).into_iter()
        // Accumulate the number of bananas per four price changes
        .fold(HashMap::new(),
              |mut acc: HashMap<u32, u64>, initial_secret| {
                  for (price_change_sequence, bananas) in bananas_per_price_change_sequence(initial_secret) {
                     match acc.entry(price_change_sequence) {
                         Vacant(entry) => { entry.insert(bananas); }
                         Occupied(mut entry) => *entry.get_mut() += bananas
                     }
                  }

                  acc
              }
        )
        .values()
        .max().unwrap()
}

#[cfg(test)]
mod tests {
    use std::iter::repeat_with;
    use super::*;

    const TEST_INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_evolve_secret_number() {
        let mut secret = 123;

        let secrets = repeat_with(|| {
            secret = evolve_secret_number(secret);
            secret
        })
        .take(10)
        .collect::<Vec<_>>();

        assert_eq!(secrets, vec![15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(37327623, part1(TEST_INPUT));
    }

    const TEST_INPUT2: &str = "1
2
3
2024";

    #[test]
    fn test_part2() {
        assert_eq!(23, part2(TEST_INPUT2));
    }
}
