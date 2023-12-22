use std::collections::HashSet;

use regex::Regex;

struct Card {
    winning: Vec<u8>,
    have: Vec<u8>,
}

fn parse(input: &str) -> Vec<Card> {
    let re = Regex::new(r"^Card\s+\d+:\s+(\d+(?:\s+\d+)+) \|\s+(\d+(?:\s+\d+)+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let winning = caps[1]
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            let have = caps[2]
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            Card { winning, have }
        })
        .collect()
}

pub fn puzzle1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|card| {
            match HashSet::<u8>::from_iter(card.winning)
                .intersection(&HashSet::<u8>::from_iter(card.have))
                .count()
            {
                0 => 0,
                n => 1 << (n - 1),
            }
        })
        .sum()
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
}
