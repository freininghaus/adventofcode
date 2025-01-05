use itertools::Itertools;
use regex::Regex;

fn input() -> String {
    std::fs::read_to_string("input/day13.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

impl ClawMachine {
    fn parse(input: &str) -> ClawMachine {
        let regex = Regex::new(r"(Button [AB]|Prize): X[+=]([0-9]+), Y[+=]([0-9]+)").unwrap();

        let (button_a, button_b, prize) = input.lines()
            .map(|line| {
                let capture = regex.captures_iter(line)
                    .next().unwrap();
                let (_, [_, x, y]) = capture.extract();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect_tuple().unwrap();

        ClawMachine{ button_a, button_b, prize}
    }
}

fn parse(data: &str) -> Vec<ClawMachine> {
    data.split("\n\n")
        .map(|three_lines| ClawMachine::parse(three_lines))
        .collect()
}

fn tokens_brute_force(machine: &ClawMachine) -> Option<i64> {
    // We could figure out how to combine the target coordinates using buttons A and B in better
    // ways, but the brute force approach is fast enough.
    (0..=100)
        .filter(|b| b * machine.button_b.0 <= machine.prize.0 && machine.button_b.1 <= machine.prize.1)
        .flat_map(|b| {
            let remainder = (
                machine.prize.0 - b * machine.button_b.0, 
                machine.prize.1 - b * machine.button_b.1);

            (0..=100)
                .filter(move |a| remainder.0 == a * machine.button_a.0 && remainder.1 == a * machine.button_a.1)
                .map(move |a| {
                    3 * a + b
                })
        })
        .min()

}

fn part1(data: &str) -> i64 {
    parse(data).iter()
        .filter_map(tokens_brute_force)
        .sum()
}

// Just for unit testing
fn part1_efficient(data: &str) -> i64 {
    parse(data).into_iter()
        .filter_map(tokens_efficient)
        .sum()
}

// For part 2, we have to do better.
// n_a * a_x + n_b * b_x = p_x  | * a_y
// n_a * a_y + n_b * b_y = p_y  | * (-a_x)
//
// n_b * (a_y * b_x - a_x * b_y) = a_y * p_x - a_x * p_y
//
// n_a = p_x / a_x - n_b * b_x / a_x
// n_a = (p_x - n_b * b_x) / a_x

fn tokens_efficient(machine: ClawMachine) -> Option<i64> {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let(px, py) = machine.prize;

    if ax * by == ay * bx {
        panic!("a and b are linearly dependent. Handle this special case!");
    }

    let nb_factor = ay * bx - ax * by;
    let nb_rhs = ay * px - ax * py;

    if nb_rhs % nb_factor != 0 {
        return None;
    }

    let nb = nb_rhs / nb_factor;

    let na_rhs_nom = px - nb * bx;

    if na_rhs_nom % ax != 0 {
        return None;
    }

    let na = na_rhs_nom / ax;

    Some(3 * na + nb)
}

fn part2(data: &str) -> i64 {
    parse(data).into_iter()
        .map(|machine| ClawMachine{
            prize: (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000),
            ..machine
        })
        .filter_map(tokens_efficient)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(480, part1(TEST_INPUT));
    }

    #[test]
    fn test_part1_efficient() {
        assert_eq!(480, part1_efficient(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(875318608908, part2(TEST_INPUT));
    }
}
