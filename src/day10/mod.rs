use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn isize_to_usize(i: isize) -> usize {
    i.try_into().unwrap()
}

// (row, col)
fn pipe(c: char) -> Option<[(isize, isize); 2]> {
    match c {
        '|' => Some([(-1, 0), (1, 0)]),
        '-' => Some([(0, -1), (0, 1)]),
        'L' => Some([(-1, 0), (0, 1)]),
        'J' => Some([(-1, 0), (0, -1)]),
        '7' => Some([(0, -1), (1, 0)]),
        'F' => Some([(0, 1), (1, 0)]),
        _ => None,
    }
}

fn main_loop(grid: &[Vec<char>]) -> Vec<(isize, isize)> {
    let get = |i: isize, j: isize| grid[isize_to_usize(i)][isize_to_usize(j)];
    let (mut i, mut j) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &c)| c == 'S')
                .map(|(j, _)| (i as isize, j as isize))
        })
        .unwrap();
    let mut path = vec![(i, j)];
    for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
        if i1 < 0
            || isize_to_usize(i1) == grid.len()
            || j1 < 0
            || isize_to_usize(j1) == grid[0].len()
        {
            continue;
        }
        if let Some(options) = pipe(get(i1, j1)) {
            if options
                .into_iter()
                .map(|(di, dj)| (i1 + di, j1 + dj))
                .any(|(i2, j2)| (i2, j2) == (i, j))
            {
                (i, j) = (i1, j1);
            }
        }
    }
    while (i, j) != path[0] {
        let &prev = path.last().unwrap();
        path.push((i, j));
        (i, j) = pipe(get(i, j))
            .unwrap()
            .into_iter()
            .map(|(di, dj)| (i + di, j + dj))
            .find(|&(i1, j1)| (i1, j1) != prev)
            .unwrap();
    }
    path
}

pub fn puzzle1(input: &str) -> usize {
    main_loop(&parse(input)).len() / 2
}

pub fn puzzle2(input: &str) -> usize {
    let grid = parse(input);
    let get = |i: isize, j: isize| grid[isize_to_usize(i)][isize_to_usize(j)];
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut edges_above = vec![vec![true; num_cols + 2]; num_rows + 2];
    for edge in edges_above[0].iter_mut() {
        *edge = false;
    }
    for edge in edges_above[num_rows + 1].iter_mut() {
        *edge = false;
    }

    let mut edges_left = vec![vec![true; num_cols + 2]; num_rows + 2];
    for row in edges_left.iter_mut() {
        row[0] = false;
        row[num_cols + 1] = false;
    }

    for (i, j) in main_loop(&grid) {
        let c = get(i, j);
        if c == 'S' {
            continue;
        }
        for (di, dj) in pipe(c).unwrap() {
            if di == 0 {
                let j1 = j.max(j + dj);
                edges_above[isize_to_usize(i + 1)][isize_to_usize(j1)] = false;
            } else {
                let i1 = i.max(i + di);
                edges_left[isize_to_usize(i1)][isize_to_usize(j + 1)] = false;
            }
        }
    }

    let mut outside = HashSet::<(usize, usize)>::new();
    let mut stack = vec![(0, 0)];
    while let Some((i, j)) = stack.pop() {
        if outside.contains(&(i, j)) {
            continue;
        }
        outside.insert((i, j));
        if edges_above[i][j] {
            stack.push((i - 1, j));
        }
        if edges_left[i][j] {
            stack.push((i, j - 1));
        }
        if edges_left[i][j + 1] {
            stack.push((i, j + 1));
        }
        if edges_above[i + 1][j] {
            stack.push((i + 1, j));
        }
    }

    let mut inside = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if !(outside.contains(&(i, j))
                || outside.contains(&(i, j + 1))
                || outside.contains(&(i + 1, j))
                || outside.contains(&(i + 1, j + 1)))
            {
                inside += 1;
            }
        }
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const EXAMPLE3: &str = include_str!("example3.txt");
    const EXAMPLE4: &str = include_str!("example4.txt");
    const EXAMPLE5: &str = include_str!("example5.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), 4);
    }

    #[test]
    fn test_puzzle1_example2() {
        assert_eq!(puzzle1(EXAMPLE2), 8);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6733);
    }

    #[test]
    fn test_puzzle2_example3() {
        assert_eq!(puzzle2(EXAMPLE3), 4);
    }

    #[test]
    fn test_puzzle2_example4() {
        assert_eq!(puzzle2(EXAMPLE4), 8);
    }

    #[test]
    fn test_puzzle2_example5() {
        assert_eq!(puzzle2(EXAMPLE5), 10);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 435);
    }
}
