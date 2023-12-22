use itertools::Itertools;

struct Range {
    dst: usize,
    src: usize,
    len: usize,
}

impl Range {
    fn map(&self, x: usize) -> Option<usize> {
        if self.src <= x && x < self.src + self.len {
            Some(x - self.src + self.dst)
        } else {
            None
        }
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn lookup(&self, x: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.map(x))
            .unwrap_or(x)
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

fn parse(input: &str) -> Almanac {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    lines.next();
    let mut maps = vec![];
    let mut map = Map { ranges: vec![] };
    for line in lines {
        if line.ends_with(" map:") {
            continue;
        }
        if line.is_empty() {
            maps.push(map);
            map = Map { ranges: vec![] };
            continue;
        }
        let (dst, src, len) = line
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        map.ranges.push(Range { dst, src, len });
    }
    maps.push(map);
    Almanac { seeds, maps }
}

pub fn puzzle1(input: &str) -> usize {
    let Almanac { seeds, maps } = parse(input);
    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |x, map| map.lookup(x)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 35);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 218513636);
    }
}
