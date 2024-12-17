fn input() -> String {
    std::fs::read_to_string("input/dayXX.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(data: &str) -> usize {
    0
}

fn part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(42, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(TEST_INPUT));
    }
}
