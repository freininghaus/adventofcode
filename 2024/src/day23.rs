use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day23.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn parse(data: &str) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    let mut insert = |a, b| {
        match result.entry(a) {
            Vacant(entry)
                => { entry.insert(vec![b].into_iter().collect()); },
            Occupied(mut entry)
                => { entry.get_mut().insert(b); }
        }
    };

    for (a, b) in data.lines()
        .map(|line| line.split('-').map(|s| s.to_owned()).collect_tuple().unwrap())
    {
        insert(a.clone(), b.clone());
        insert(b, a);
    }

    result
}

fn part1(data: &str) -> usize {
    let connections = parse(data);

    let mut result: usize = 0;

    for (a, peers) in connections.iter() {
        for b in peers {
            if b <= a {
                continue;
            }

            for c in peers.intersection(&connections[b]) {
                if c <= b {
                    continue;
                }

                if [a, b, c].iter()
                    .any(|computer| computer.chars().next() == Some('t')) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
