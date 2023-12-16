#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    LeftUpMirror,
    RightUpMirror,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    pub energized: bool,
    /// 0=L, 1=R, 2=U, 3=D
    pub energy_directions: [bool; 4],
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            energized: false,
            energy_directions: [false; 4],
        }
    }

    pub fn reset(&mut self) {
        self.energized = false;
        self.energy_directions = [false; 4];
    }

    /// Energize the tile and return the direction the beam should go
    pub fn energize(&mut self, bean_direction: Movement) -> (Movement, Option<Movement>) {
        self.energized = true;
        let direction = match bean_direction {
            Movement::Up => 2,
            Movement::Down => 3,
            Movement::Left => 0,
            Movement::Right => 1,
        };
        self.energy_directions[direction] = true;
        match self.tile_type {
            TileType::HorizontalSplitter => match bean_direction {
                Movement::Left => (Movement::Left, None),
                Movement::Right => (Movement::Right, None),
                _ => (Movement::Left, Some(Movement::Right)),
            },
            TileType::VerticalSplitter => match bean_direction {
                Movement::Up => (Movement::Up, None),
                Movement::Down => (Movement::Down, None),
                _ => (Movement::Up, Some(Movement::Down)),
            },
            TileType::LeftUpMirror => match bean_direction {
                Movement::Up => (Movement::Right, None),
                Movement::Down => (Movement::Left, None),
                Movement::Left => (Movement::Down, None),
                Movement::Right => (Movement::Up, None),
            },
            TileType::RightUpMirror => match bean_direction {
                Movement::Up => (Movement::Left, None),
                Movement::Down => (Movement::Right, None),
                Movement::Left => (Movement::Up, None),
                Movement::Right => (Movement::Down, None),
            },
            TileType::Empty => (bean_direction, None),
        }
    }

    pub fn direction_already_energized(&self, direction: Movement) -> bool {
        match direction {
            Movement::Up => self.energy_directions[2],
            Movement::Down => self.energy_directions[3],
            Movement::Left => self.energy_directions[0],
            Movement::Right => self.energy_directions[1],
        }
    }
}
