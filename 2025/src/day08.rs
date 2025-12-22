use std::collections::HashSet;
use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("../inputs/2025/day08.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data, 1000));
    println!("Part 2: {}", part2(&data));
}

fn part1(lines: &str, connections: usize) -> usize {
    connect_boxes(lines, connections).0
        .iter()
        .map(|boxes| boxes.len())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

fn part2(lines: &str) -> i64 {
    let box_count = lines.lines().count();

    if let Some((last_box_1, last_box_2)) = connect_boxes(lines, box_count * box_count).1 {
        last_box_1.0 * last_box_2.0
    } else {
        panic!("No boxes found");
    }
}

fn connect_boxes(
    lines: &str,
    max_connections: usize,
) -> (
    Vec<HashSet<usize>>,
    Option<((i64, i64, i64), (i64, i64, i64))>,
) {
    let positions = parse(lines);
    let sorted_squared_distances: Vec<(u64, (usize, usize))> = positions
        .iter()
        .enumerate()
        .flat_map(|(index1, (x1, y1, z1))| {
            positions
                .iter()
                .take(index1)
                .enumerate()
                .map(move |(index2, (x2, y2, z2))| {
                    (
                        ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2))
                            as u64,
                        (index1, index2),
                    )
                })
        })
        .sorted()
        .collect();

    let mut circuit_id_for_box_index: Vec<Option<usize>> = vec![None; positions.len()];
    let mut box_indices_in_circuit_by_id: Vec<HashSet<usize>> = Vec::new();
    let mut last_connection: Option<((i64, i64, i64), (i64, i64, i64))> = None;

    for (_, (box1, box2)) in sorted_squared_distances.into_iter().take(max_connections) {
        if let Some(circuit1) = circuit_id_for_box_index[box1]
            && circuit_id_for_box_index[box2].is_none()
        {
            // Add single box 2 to circuit of box 1
            circuit_id_for_box_index[box2] = Some(circuit1);
            box_indices_in_circuit_by_id[circuit1].insert(box2);
        } else if let Some(circuit2) = circuit_id_for_box_index[box2]
            && circuit_id_for_box_index[box1].is_none()
        {
            // Add single box 1 to circuit of box 2
            circuit_id_for_box_index[box1] = Some(circuit2);
            box_indices_in_circuit_by_id[circuit2].insert(box1);
        } else if circuit_id_for_box_index[box1].is_none()
            && circuit_id_for_box_index[box2].is_none()
        {
            // Build new circuit out of box 1 and box 2
            let new_circuit_id = box_indices_in_circuit_by_id.len();
            circuit_id_for_box_index[box1] = Some(new_circuit_id);
            circuit_id_for_box_index[box2] = Some(new_circuit_id);
            box_indices_in_circuit_by_id.push(HashSet::from([box1, box2]));
        } else if let Some(mut circuit1) = circuit_id_for_box_index[box1]
            && let Some(mut circuit2) = circuit_id_for_box_index[box2]
        {
            // Both boxes are part of a circuit already
            if circuit1 == circuit2 {
                // Both boxes are part of the same circuit
                continue;
            }

            // Merge the circuits that box 1 and box 2 are part of.
            // Make sure that the circuit with the lower index persists, such that the single final
            // circuit will be the one with index zero.
            if circuit2 < circuit1 {
                (circuit1, circuit2) = (circuit2, circuit1);
            }

            for box_index in box_indices_in_circuit_by_id[circuit2].clone() {
                circuit_id_for_box_index[box_index] = Some(circuit1);
                box_indices_in_circuit_by_id[circuit1].insert(box_index);
            }

            box_indices_in_circuit_by_id[circuit2].clear();
        }

        if box_indices_in_circuit_by_id.iter().next().unwrap().len() == positions.len() {
            // All boxes are connected in a single circuit
            last_connection = Some((positions[box1].clone(), positions[box2].clone()));
            break;
        }
    }

    (box_indices_in_circuit_by_id, last_connection)
}

fn parse(lines: &str) -> Vec<(i64, i64, i64)> {
    lines
        .lines()
        .map(|line| {
            line.split(",")
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(40, part1(TEST_INPUT, 10));
    }

    #[test]
    fn test_part2() {
        assert_eq!(25272, part2(TEST_INPUT));
    }
}
