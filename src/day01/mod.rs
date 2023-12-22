pub fn puzzle1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum()
}

const DIGITS: [(u32, [&str; 2]); 9] = [
    (1, ["1", "one"]),
    (2, ["2", "two"]),
    (3, ["3", "three"]),
    (4, ["4", "four"]),
    (5, ["5", "five"]),
    (6, ["6", "six"]),
    (7, ["7", "seven"]),
    (8, ["8", "eight"]),
    (9, ["9", "nine"]),
];

pub fn puzzle2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = (0..=line.len())
                .find_map(|i| {
                    let suffix = &line[i..];
                    DIGITS
                        .iter()
                        .find(|(_, words)| words.iter().any(|word| suffix.starts_with(word)))
                        .map(|(digit, _)| digit)
                })
                .unwrap();
            let last = (0..=line.len())
                .rev()
                .find_map(|i| {
                    let prefix = &line[..i];
                    DIGITS
                        .iter()
                        .find(|(_, words)| words.iter().any(|word| prefix.ends_with(word)))
                        .map(|(digit, _)| digit)
                })
                .unwrap();
            10 * first + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), 142);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 54597);
    }

    #[test]
    fn test_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE2), 281);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 54504);
    }
}
