use regex::Regex;

struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse(input: &str) -> Vec<Vec<Set>> {
    let re_line = Regex::new(r"^Game \d+: (.*)$").unwrap();
    let re_kind = Regex::new(r"^(\d+) (\w+)$").unwrap();
    input
        .lines()
        .map(|line| {
            re_line.captures(line).unwrap()[1]
                .split("; ")
                .map(|set| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    for kind in set.split(", ") {
                        let caps = re_kind.captures(kind).unwrap();
                        let count = caps[1].parse().unwrap();
                        match &caps[2] {
                            "red" => red = count,
                            "green" => green = count,
                            "blue" => blue = count,
                            _ => unreachable!(),
                        }
                    }
                    Set { red, green, blue }
                })
                .collect()
        })
        .collect()
}

pub fn puzzle1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        })
        .map(|(i, _)| i + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 8);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 2283);
    }
}
