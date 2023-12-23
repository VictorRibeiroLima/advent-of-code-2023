use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tile {
    i: usize,
    j: usize,
}

struct Map {
    tiles: Vec<Vec<Option<Tile>>>,
    intersections: Vec<Tile>,
    graph: HashMap<Tile, Vec<(Tile, usize)>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let tile = match c {
                    '#' => None,
                    _ => Some(Tile { i, j }),
                };
                row.push(tile);
            }
            tiles.push(row);
        }
        let mut map = Map {
            tiles,
            intersections: Vec::new(),
            graph: HashMap::new(),
        };
        let intersections = map.map_intersections();
        map.intersections = intersections;
        let graph = map.graph_intersection();
        map.graph = graph;
        map
    }

    fn dfs(&self, start: Tile, end: Tile, seen: &mut HashSet<Tile>) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        seen.insert(start);
        let mut max = None;
        let tile_graph = self.graph.get(&start).unwrap();
        for (tile, steps) in tile_graph {
            if !seen.contains(tile) {
                let other_dfs = self.dfs(*tile, end, seen);
                if other_dfs.is_none() {
                    continue;
                }
                let other_dfs = other_dfs.unwrap();
                let total = steps + other_dfs;
                if max == None || total > max.unwrap() {
                    max = Some(total);
                }
            }
        }
        seen.remove(&start);
        max
    }

    fn graph_intersection(&self) -> HashMap<Tile, Vec<(Tile, usize)>> {
        let mut graph = HashMap::new();
        for intersection in &self.intersections {
            let mut stack = Vec::new();
            stack.push((0, *intersection));
            let mut visited = HashSet::new();
            while let Some((steps, tile)) = stack.pop() {
                if self.intersections.contains(&tile) && tile != *intersection {
                    graph
                        .entry(*intersection)
                        .or_insert_with(Vec::new)
                        .push((tile, steps));
                    continue;
                }

                let options = self.get_possible_tiles(tile);
                for option in options {
                    if !visited.contains(&option) {
                        visited.insert(option);
                        stack.push((steps + 1, option));
                    }
                }
            }
        }
        graph
    }

    fn map_intersections(&self) -> Vec<Tile> {
        let mut intersections = Vec::new();
        intersections.push(Tile { i: 0, j: 1 });
        for r in &self.tiles {
            for tile in r {
                if let Some(tile) = tile {
                    let options = self.get_possible_tiles(*tile);
                    if options.len() > 2 {
                        intersections.push(*tile);
                    }
                }
            }
        }
        intersections.push(Tile {
            i: self.tiles.len() - 1,
            j: self.tiles[0].len() - 2,
        });
        intersections
    }

    fn walk_longest(&self) -> usize {
        let start = self.get_tile(0, 1).unwrap();
        let end_i = (self.tiles.len() - 1) as isize;
        let end_j = (self.tiles[0].len() - 2) as isize;
        let end = self.get_tile(end_i, end_j).unwrap();
        self.dfs(start, end, &mut HashSet::new()).unwrap()
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

    fn get_possible_tiles(&self, tile: Tile) -> Vec<Tile> {
        let mut tiles = Vec::new();
        let i = tile.i;
        let j = tile.j;

        let up = self.get_tile(i as isize - 1, j as isize);
        let down = self.get_tile(i as isize + 1, j as isize);
        let left = self.get_tile(i as isize, j as isize - 1);
        let right = self.get_tile(i as isize, j as isize + 1);
        if let Some(up) = up {
            tiles.push(up);
        }
        if let Some(left) = left {
            tiles.push(left);
        }
        if let Some(down) = down {
            tiles.push(down);
        }

        if let Some(right) = right {
            tiles.push(right);
        }

        tiles
    }
}

pub fn process(input: &str) -> usize {
    let map = Map::new(input);
    let distance = map.walk_longest();
    distance
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersections() {
        let input = include_str!("./inputs/test.txt");
        let map = Map::new(input);
        assert_eq!(map.intersections.len(), 9);
    }

    #[test]
    fn test_graph() {
        let input = include_str!("./inputs/test.txt");
        let map = Map::new(input);
        let graph = map.graph_intersection();
        println!("{:#?}", graph);
    }

    #[test]
    fn test_walk_longest() {
        let input = include_str!("./inputs/test.txt");
        let map = Map::new(input);
        let distance = map.walk_longest();
        assert_eq!(distance, 154);
    }

    #[test]
    fn input_walk_longest() {
        let input = include_str!("./inputs/input.txt");
        let map = Map::new(input);

        let distance = map.walk_longest();
        assert_eq!(distance, 6874);
    }
}
