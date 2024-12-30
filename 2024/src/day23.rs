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

fn find_large_networks<F: FnMut(&Vec<String>) -> ()>(
    connections: &HashMap<String, HashSet<String>>,
    candidates: &HashSet<String>,
    network_start: &mut Vec<String>,
    consumer: &mut F
) {
    if candidates.is_empty() {
        // Cannot grow the network any more
        consumer(network_start);
    }

    // Grow the network by every possible computer and continue to find further ways to grow it
    for computer in candidates {
        network_start.push(computer.clone());

        // We start with the previous set of candidates, i.e. the computers which are connected to
        // each computer in network_start
        let new_candidates: HashSet<String> = candidates.iter()
            // New candidates must be connected to the chosen computer
            .filter(|next_computer| connections[computer].contains(*next_computer))
            // To avoid visiting the same network multiple times, we enforce that computers are
            // sorted in ascending order by name
            .filter(|next_computer| *next_computer > computer)
            .cloned()
            .collect();

        find_large_networks(connections, &new_candidates, network_start, consumer);

        network_start.pop();
    }
}

fn part2(data: &str) -> String {
    let connections = parse(data);
    let all_computers: HashSet<String> = connections.keys().cloned().collect();

    let mut large_networks: Vec<Vec<String>> = Vec::new();
    let mut network: Vec<String> = Vec::new();

    find_large_networks(
        &connections,
        &all_computers,
        &mut network,
        &mut |large_network: &Vec<String>| large_networks.push(large_network.clone())
    );

    large_networks.into_iter()
        .max_by_key(|l| l.len())
        .unwrap()
        .iter().join(",")
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
        assert_eq!("co,de,ka,ta", part2(TEST_INPUT));
    }
}
