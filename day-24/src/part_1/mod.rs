use std::collections::{BTreeSet, HashSet};

use self::hailstone::HailStone;

const MIN: i64 = 200_000_000_000_000;
const MAX: i64 = 400_000_000_000_000;

mod hailstone;
mod line;

struct Map {
    stones: Vec<HailStone>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut stones = Vec::new();
        for line in input.lines() {
            stones.push(HailStone::new(line, (MIN, MAX)));
        }
        Self { stones }
    }

    fn count_intersections(&self) -> usize {
        let mut result = 0;
        for i in 0..self.stones.len() {
            let mut count = 0;
            let stone = &self.stones[i];
            for j in i + 1..self.stones.len() {
                let other = &self.stones[j];
                let intersection = stone.paths_intersect(other);
                match intersection {
                    Some(_) => {
                        count += 1;
                    }
                    None => {}
                }
            }
            println!("Stone {} has {} intersections", i, count);
            result += count;
        }
        result
    }

    #[cfg(test)]
    fn new_test(input: &str) -> Self {
        let mut stones = Vec::new();
        for line in input.lines() {
            stones.push(HailStone::new(line, (7, 27)));
        }
        Self { stones }
    }
}

pub fn process(input: &str) -> usize {
    let map = Map::new(input);
    map.count_intersections()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let input = include_str!("../inputs/test.txt");
        let map = Map::new_test(input);
        assert_eq!(map.stones.len(), 5);
    }

    #[test]
    fn test_new_2() {
        let input = include_str!("../inputs/input.txt");
        let map = Map::new(input);
        assert_eq!(map.stones.len(), 300);
    }

    #[test]
    fn test_count_intersections() {
        let input = include_str!("../inputs/test.txt");
        let map = Map::new_test(input);
        assert_eq!(map.count_intersections(), 2);
    }
}
