use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

fn adjacencies(pred: impl Fn(char) -> bool, lines: &[&str]) -> Vec<Vec<Vec<(usize, usize)>>> {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut adjacent = vec![vec![vec![]; cols]; rows];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if pred(c) {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let i1 = i as isize + di;
                        let j1 = j as isize + dj;
                        if i1 >= 0 && i1 < rows as isize && j1 >= 0 && j1 < cols as isize {
                            adjacent[i1 as usize][j1 as usize].push((i, j));
                        }
                    }
                }
            }
        }
    }
    adjacent
}

fn numbers(mut f: impl FnMut(usize, usize, usize, usize), lines: &[&str]) {
    for (i, line) in lines.iter().enumerate() {
        let mut start: Option<usize> = None;
        for (j, c) in line.chars().map(Some).chain(once(None)).enumerate() {
            if c.is_none() || !c.unwrap().is_ascii_digit() {
                if let Some(k) = start {
                    f(i, k, j, line[k..j].parse::<usize>().unwrap());
                }
                start = None;
            } else if start.is_none() {
                start = Some(j);
            }
        }
    }
}

pub fn puzzle1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let adjacent = adjacencies(|c| !(c == '.' || c.is_ascii_digit()), &lines);
    let mut sum = 0;
    numbers(
        |i, k, j, n| {
            if (k..j).any(|l| !adjacent[i][l].is_empty()) {
                sum += n;
            }
        },
        &lines,
    );
    sum
}

pub fn puzzle2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let adjacent = adjacencies(|c| c == '*', &lines);
    let mut gears = HashMap::<(usize, usize), Vec<usize>>::new();
    numbers(
        |i, k, j, n| {
            let mut stars = HashSet::<(usize, usize)>::new();
            for l in k..j {
                stars.extend(adjacent[i][l].iter());
            }
            for star in stars {
                gears.entry(star).or_default().push(n);
            }
        },
        &lines,
    );
    gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 4361);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 538046);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 467835);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 81709807);
    }
}
