use std::collections::HashSet;

use regex::Regex;

fn parse(input: &str) -> Vec<usize> {
    let re = Regex::new(r"^Card\s+\d+:\s+(\d+(?:\s+\d+)+) \|\s+(\d+(?:\s+\d+)+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let winning: HashSet<u8> = caps[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let have: HashSet<u8> = caps[2]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            winning.intersection(&have).count()
        })
        .collect()
}

pub fn puzzle1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|n| if n > 0 { 1 << (n - 1) } else { 0 })
        .sum()
}

pub fn puzzle2(input: &str) -> usize {
    let cards = parse(input);
    let n = cards.len();
    let mut counts = vec![1; n];
    for (i, n) in cards.into_iter().enumerate() {
        for j in (i + 1)..=(i + n) {
            counts[j] += counts[i];
        }
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 13);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 22488);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 30);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 7013204);
    }
}
