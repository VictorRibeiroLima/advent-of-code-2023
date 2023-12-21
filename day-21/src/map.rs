use std::{
    collections::HashSet,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    walkable: bool,
    start: bool,
    possible_next: bool,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Location {
    pub i: usize,
    pub j: usize,
}
impl Location {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
    }
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut start = None;

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile {
                        walkable: true,
                        start: false,
                        possible_next: false,
                    },
                    '#' => Tile {
                        walkable: false,
                        start: false,
                        possible_next: false,
                    },
                    'S' => {
                        start = Some((x, y));
                        Tile {
                            walkable: true,
                            start: true,
                            possible_next: false,
                        }
                    }
                    _ => panic!("Invalid tile: {}", c),
                };
                row.push(tile);
            }
            tiles.push(row);
        }

        let start = start.expect("No start found");
        let mut on_locations = HashSet::new();
        on_locations.insert(Location {
            i: start.0,
            j: start.1,
        });

        Self { tiles, start }
    }

    pub fn reset(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                tile.possible_next = false;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.tiles.len()
    }

    pub fn walk(&mut self, num_of_steps: usize) -> usize {
        let start = self.start_location();
        let mut count = 0;
        let on_locations = vec![start].into_iter().collect();
        self.inner_walk(on_locations, num_of_steps);
        for row in &self.tiles {
            for tile in row {
                if tile.possible_next {
                    count += 1;
                }
            }
        }
        self.reset();
        count
    }

    pub fn walk_from_location(&mut self, location: Location, num_of_steps: usize) -> usize {
        let on_locations = vec![location].into_iter().collect();
        self.inner_walk(on_locations, num_of_steps);
        let mut count = 0;
        for row in &self.tiles {
            for tile in row {
                if tile.possible_next {
                    count += 1;
                }
            }
        }
        self.reset();
        count
    }

    pub fn start_location(&self) -> Location {
        Location {
            i: self.start.0,
            j: self.start.1,
        }
    }

    fn inner_walk(&mut self, on_locations: HashSet<Location>, num_of_steps: usize) {
        if num_of_steps == 0 {
            for location in on_locations {
                let possible_locations = self.check_possible_moves(location);
                for possible_location in possible_locations {
                    self.tiles[possible_location.j][possible_location.i].possible_next = true;
                }
            }
            return;
        }

        let mut new_on_locations: HashSet<Location> = HashSet::new();
        for location in on_locations {
            let possible_locations = self.check_possible_moves(location);
            for possible_location in possible_locations {
                new_on_locations.insert(possible_location);
            }
        }
        self.inner_walk(new_on_locations, num_of_steps - 1);
    }

    fn check_possible_moves(&self, location: Location) -> Vec<Location> {
        let mut result = Vec::new();
        //check up
        if location.i > 0 {
            let tile = self.tiles[location.i - 1][location.j];
            if tile.walkable {
                result.push(Location {
                    i: location.i - 1,
                    j: location.j,
                });
            }
        }
        //check down
        if location.i < self.tiles.len() - 1 {
            let tile = self.tiles[location.i + 1][location.j];
            if tile.walkable {
                result.push(Location {
                    i: location.i + 1,
                    j: location.j,
                });
            }
        }
        //check left
        if location.j > 0 {
            let tile = self.tiles[location.i][location.j - 1];
            if tile.walkable {
                result.push(Location {
                    i: location.i,
                    j: location.j - 1,
                });
            }
        }
        //check right
        if location.j < self.tiles[location.i].len() - 1 {
            let tile = self.tiles[location.i][location.j + 1];
            if tile.walkable {
                result.push(Location {
                    i: location.i,
                    j: location.j + 1,
                });
            }
        }

        return result;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let c = if tile.walkable {
                    if tile.possible_next {
                        'O'
                    } else if tile.start {
                        'S'
                    } else {
                        '.'
                    }
                } else {
                    '#'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        let map = Map::new(input);
        let map_str = format!("{}", map);
        assert_eq!(map_str, input)
    }

    #[test]
    fn test_0_step() {
        let input = include_str!("./inputs/test.txt");
        let mut map = Map::new(input);
        let result = map.walk(0);
        println!("{}", map);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_1_step() {
        let input = include_str!("./inputs/test.txt");
        let mut map = Map::new(input);
        let result = map.walk(1);
        println!("{}", map);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2_step() {
        let input = include_str!("./inputs/test.txt");
        let mut map = Map::new(input);
        let result = map.walk(2);
        println!("{}", map);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_5_step() {
        let input = include_str!("./inputs/test.txt");
        let mut map = Map::new(input);
        let result = map.walk(5);
        println!("{}", map);
        assert_eq!(result, 16);
    }
}
