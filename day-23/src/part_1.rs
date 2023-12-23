use std::{collections::BinaryHeap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    All,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tile {
    i: usize,
    j: usize,
    direction: Direction,
}

struct Map {
    tiles: Vec<Vec<Option<Tile>>>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Some(Tile {
                        direction: Direction::All,
                        i,
                        j,
                    }),
                    '^' => Some(Tile {
                        direction: Direction::Up,
                        i,
                        j,
                    }),
                    'v' => Some(Tile {
                        direction: Direction::Down,
                        i,
                        j,
                    }),
                    '<' => Some(Tile {
                        direction: Direction::Left,
                        i,
                        j,
                    }),
                    '>' => Some(Tile {
                        direction: Direction::Right,
                        i,
                        j,
                    }),
                    _ => None,
                };
                row.push(tile);
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    pub fn walk_longest(&self, start: Tile, end: (usize, usize)) -> usize {
        let mut heap = BinaryHeap::new();
        self.inner_walk(start, end, start, 0, &mut heap);
        heap.pop().unwrap()
    }

    fn inner_walk(
        &self,
        start: Tile,
        end: (usize, usize),
        coming_from: Tile,
        steps: usize,
        heap: &mut BinaryHeap<usize>,
    ) {
        let options = self.get_possible_tiles(start, coming_from);
        for option in options {
            if option.i == end.0 && option.j == end.1 {
                heap.push(steps + 1);
            } else {
                self.inner_walk(option, end, start, steps + 1, heap);
            }
        }
    }

    fn get_tile(&self, i: isize, j: isize) -> Option<Tile> {
        if i < 0 || j < 0 {
            return None;
        }
        if i >= self.tiles.len() as isize {
            return None;
        }
        if j >= self.tiles[0].len() as isize {
            return None;
        }
        let row = self.tiles.get(i as usize)?;
        let tile = row.get(j as usize)?;
        *tile
    }

    fn get_possible_tiles(&self, tile: Tile, coming_from: Tile) -> Vec<Tile> {
        let mut tiles = Vec::new();
        let i = tile.i;
        let j = tile.j;
        match tile.direction {
            Direction::All => {
                let up = self.get_tile(i as isize - 1, j as isize);
                let down = self.get_tile(i as isize + 1, j as isize);
                let left = self.get_tile(i as isize, j as isize - 1);
                let right = self.get_tile(i as isize, j as isize + 1);
                if let Some(up) = up {
                    if up != coming_from {
                        let up_direction = up.direction;
                        if up_direction != Direction::Down {
                            tiles.push(up);
                        }
                    }
                }
                if let Some(left) = left {
                    if left != coming_from {
                        let left_direction = left.direction;
                        if left_direction != Direction::Right {
                            tiles.push(left);
                        }
                    }
                }
                if let Some(down) = down {
                    if down != coming_from {
                        let down_direction = down.direction;
                        if down_direction != Direction::Up {
                            tiles.push(down);
                        }
                    }
                }

                if let Some(right) = right {
                    if right != coming_from {
                        let right_direction = right.direction;
                        if right_direction != Direction::Left {
                            tiles.push(right);
                        }
                    }
                }
            }
            Direction::Up => {
                let up = self.get_tile(i as isize - 1, j as isize);
                if let Some(up) = up {
                    tiles.push(up);
                }
            }
            Direction::Down => {
                let down = self.get_tile(i as isize + 1, j as isize);
                if let Some(down) = down {
                    tiles.push(down);
                }
            }
            Direction::Left => {
                let left = self.get_tile(i as isize, j as isize - 1);
                if let Some(left) = left {
                    tiles.push(left);
                }
            }
            Direction::Right => {
                let right = self.get_tile(i as isize, j as isize + 1);
                if let Some(right) = right {
                    tiles.push(right);
                }
            }
        }
        tiles
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                if let Some(tile) = tile {
                    let c = match tile.direction {
                        Direction::All => '.',
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };
                    write!(f, "{}", c)?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn process(input: &str) -> usize {
    let map = Map::new(input);
    let start = map.get_tile(0, 1).unwrap();
    let i_len = map.tiles.len();
    let j_len = map.tiles[0].len();
    let end = (i_len - 1, j_len - 2);
    let distance = map.walk_longest(start, end);
    distance
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_longest() {
        let input = include_str!("./inputs/test.txt");
        let map = Map::new(input);
        let start = map.get_tile(0, 1).unwrap();
        let i_len = map.tiles.len();
        let j_len = map.tiles[0].len();
        let end = (i_len - 1, j_len - 2);
        let distance = map.walk_longest(start, end);
        assert_eq!(distance, 94);
    }

    #[test]
    fn input_walk_longest() {
        let input = include_str!("./inputs/input.txt");
        let map = Map::new(input);
        let start = map.get_tile(0, 1).unwrap();
        let i_len = map.tiles.len();
        let j_len = map.tiles[0].len();
        let end = (i_len - 1, j_len - 2);
        let distance = map.walk_longest(start, end);
        assert_eq!(distance, 2314);
    }
}
