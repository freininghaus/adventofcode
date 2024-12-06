use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/day04.txt")
        .expect("Could not read file")
}

fn main() {
    let data = input();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

struct WordSearch {
    n: usize,
    data: Vec<Vec<char>>
}

impl WordSearch {
    fn new(lines: &str) -> Self {
        let data: Vec<Vec<char>> = lines.lines()
            .map(|line| line.chars().filter(|&c| c.is_ascii_alphabetic()).collect())
            .collect();
        
        let n = data.len();
        
        if !data.iter().all(|row| row.len() == n) {
            panic!("Expected a square grid of letters");
        }
        
        Self { n, data }
    }

    fn line(&self, points: &Vec<(usize, usize)>) -> String {
        points.into_iter().map(|(x, y)| self.data[*y][*x]).collect()
    }
}

fn part1(lines: &str) -> usize {
    let word_search = WordSearch::new(lines); 
    let n = word_search.n;

    let rows = (0..n).map(|y| (0..n).map(|x| (x, y)).collect());
    let columns = (0..n).map(|x| (0..n).map(|y| (x, y)).collect());
    
    let diag_down_1 = (0..n).map(|x0| (x0..n).map(|x| (x, x - x0)).collect());
    let diag_down_2 = (1..n).map(|y0| (0..n - y0).map(|x| (x, y0 + x)).collect());
    
    let diag_up_1 = (0..n).map(|x0| (x0..n).map(|x| (x, n - 1 - (x - x0))).collect());
    let diag_up_2 = (0..n - 1).map(|y0| (0..y0 + 1).map(|x| (x, y0 - x)).collect());


    let all_lines: Vec<Vec<_>> = rows.chain(columns).chain(diag_down_1).chain(diag_down_2).chain(diag_up_1).chain(diag_up_2).collect();
    
    let all_words: Vec<String> = all_lines.iter().map(|points| word_search.line(&points)).collect();
    
    let haystack = all_words.join(" ");
    
    haystack.match_indices("XMAS").count() + haystack.match_indices("SAMX").count()
}

fn part2(lines: &str) -> usize {
    let word_search: Vec<Vec<char>> = lines.lines()
        .map(|line| line.chars().filter(|&c| c.is_ascii_alphabetic()).collect())
        .collect::<Vec<_>>();

    let x_mas = 
        |x: usize, y: usize| word_search[y][x] == 'A'
            && (-1..=1).map(|i| word_search[(y as i32 + i) as usize][(x as i32 + i) as usize]).sorted().collect::<String>() == "AMS"
            && (-1..=1).map(|i| word_search[(y as i32 + i) as usize][(x as i32 - i) as usize]).sorted().collect::<String>() == "AMS";

    let width = word_search[0].len();
    let height = word_search.len();

    (1..width - 1).map(|x|
        (1..height - 1).filter(|y| x_mas(x, *y)).count()
    ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(TEST_INPUT));
    }
}
