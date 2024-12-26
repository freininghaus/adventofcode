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

fn part2(data: &str) -> usize {
    0
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

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
