use std::iter::once;

pub fn puzzle1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut adjacent = vec![vec![false; cols]; rows];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if !(c == '.' || c.is_ascii_digit()) {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let i1 = i as isize + di;
                        let j1 = j as isize + dj;
                        if i1 >= 0 && i1 < rows as isize && j1 >= 0 && j1 < cols as isize {
                            adjacent[i1 as usize][j1 as usize] = true;
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut start: Option<usize> = None;
        for (j, c) in line.chars().map(Some).chain(once(None)).enumerate() {
            if j == cols || !c.unwrap().is_ascii_digit() {
                if let Some(k) = start {
                    if (k..j).any(|l| adjacent[i][l]) {
                        sum += line[k..j].parse::<usize>().unwrap();
                    }
                }
                start = None;
            } else if start.is_none() {
                start = Some(j);
            }
        }
    }

    sum
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
}
