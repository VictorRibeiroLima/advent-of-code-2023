use std::fmt::Display;

use self::tile::{Movement, Tile, TileType};

pub mod tile;

type Location = (isize, isize);

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let tile_type = match c {
                    '-' => Tile::new(TileType::HorizontalSplitter),
                    '|' => Tile::new(TileType::VerticalSplitter),
                    '/' => Tile::new(TileType::LeftUpMirror),
                    '\\' => Tile::new(TileType::RightUpMirror),
                    _ => Tile::new(TileType::Empty),
                };
                row.push(tile_type);
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    pub fn reset(&mut self) {
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                tile.reset();
            }
        }
    }

    pub fn energize(&mut self) {
        let initial_direction = Movement::Right;
        let initial_location = (0, 0);
        self.energize_location(initial_location, initial_direction);
    }

    pub fn energize_from(&mut self, location: Location, direction: Movement) {
        self.energize_location(location, direction);
    }

    pub fn count_energized(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|tile| tile.energized).count())
            .sum()
    }

    fn energize_location(&mut self, location: Location, direction: Movement) {
        if location.0 < 0 || location.1 < 0 {
            return;
        }
        if location.0 >= self.tiles.len() as isize || location.1 >= self.tiles[0].len() as isize {
            return;
        }
        let tile = &mut self.tiles[location.0 as usize][location.1 as usize];
        if tile.direction_already_energized(direction) {
            return;
        }
        let (new_direction, new_beam_direction) = tile.energize(direction);
        let location1 = match new_direction {
            Movement::Up => (location.0 - 1, location.1),
            Movement::Down => (location.0 + 1, location.1),
            Movement::Left => (location.0, location.1 - 1),
            Movement::Right => (location.0, location.1 + 1),
        };
        self.energize_location(location1, new_direction);
        if let Some(new_beam_direction) = new_beam_direction {
            let location2 = match new_beam_direction {
                Movement::Up => (location.0 - 1, location.1),
                Movement::Down => (location.0 + 1, location.1),
                Movement::Left => (location.0, location.1 - 1),
                Movement::Right => (location.0, location.1 + 1),
            };
            self.energize_location(location2, new_beam_direction);
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                if tile.energized {
                    write!(f, "#")?;
                    continue;
                }
                let c = match tile.tile_type {
                    TileType::Empty => '.',
                    TileType::HorizontalSplitter => '-',
                    TileType::VerticalSplitter => '|',
                    TileType::LeftUpMirror => '/',
                    TileType::RightUpMirror => '\\',
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
    fn test_grid_new() {
        let input = include_str!("../inputs/test.txt");
        let grid = Grid::new(input);
        let grid_str = grid.to_string();
        assert_eq!(grid_str, input);
    }

    #[test]
    fn test_energy() {
        let input = include_str!("../inputs/test.txt");
        let mut grid = Grid::new(input);
        grid.energize();
        let energized = grid.count_energized();
        assert_eq!(energized, 46);
    }
}
