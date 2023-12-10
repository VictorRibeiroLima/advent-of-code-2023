use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Up,
    Down,
    Left,
    Right,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            Side::Up => Side::Down,
            Side::Down => Side::Up,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    pub fn is_opposite(&self, other: &Side) -> bool {
        match self {
            Side::Up => match other {
                Side::Down => true,
                _ => false,
            },
            Side::Down => match other {
                Side::Up => true,
                _ => false,
            },
            Side::Left => match other {
                Side::Right => true,
                _ => false,
            },
            Side::Right => match other {
                Side::Left => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PipeType {
    Horizontal,
    Vertical,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

/*

'|' Up/Down
 '-' Left/Right
 'L' Up/Right
 'J' Left/UP
 '7' Left/Down
 'F' Right/Down
*/
#[derive(Debug, Clone, Copy)]
pub struct Pipe {
    pub first_side: Side,
    pub second_side: Side,
    pub first_side_connected: bool,
    pub second_side_connected: bool,
    pub is_start: bool,
    pub main_loop_count: Option<usize>,
    pub pipe_type: PipeType,
}

impl Pipe {
    pub fn new(first_side: Side, second_side: Side) -> Pipe {
        let pipe_type = Pipe::gen_type(first_side, second_side);
        Pipe {
            first_side,
            second_side,
            first_side_connected: false,
            second_side_connected: false,
            is_start: false,
            main_loop_count: None,
            pipe_type,
        }
    }

    pub fn try_connect(&mut self, pipe: &mut Pipe) {
        let first_side = self.first_side;
        let second_side = self.second_side;
        if pipe.first_side.is_opposite(&first_side) && !pipe.first_side_connected {
            self.first_side_connected = true;
            pipe.first_side_connected = true;
        } else if pipe.first_side.is_opposite(&second_side) && !pipe.first_side_connected {
            self.second_side_connected = true;
            pipe.first_side_connected = true;
        } else if pipe.second_side.is_opposite(&first_side) {
            self.first_side_connected = true;
            pipe.second_side_connected = true;
        } else if pipe.second_side.is_opposite(&second_side) {
            self.second_side_connected = true;
            pipe.second_side_connected = true;
        }
    }

    pub fn get_clockwise_side(&self) -> Side {
        let pipe_type = self.pipe_type;
        match pipe_type {
            PipeType::Horizontal => Side::Right,
            PipeType::Vertical => Side::Down,
            PipeType::LeftUp => Side::Up,
            PipeType::LeftDown => Side::Down,
            PipeType::RightUp => Side::Right,
            PipeType::RightDown => Side::Right,
        }
    }

    pub fn get_counterclockwise_side(&self) -> Side {
        let pipe_type = self.pipe_type;
        match pipe_type {
            PipeType::LeftDown => Side::Left,
            PipeType::LeftUp => Side::Left,
            PipeType::RightDown => Side::Down,
            PipeType::RightUp => Side::Up,
            PipeType::Horizontal => Side::Left,
            PipeType::Vertical => Side::Up,
        }
    }

    /*
       PipeType::Horizontal => write!(f, "─"),
           PipeType::Vertical => write!(f, "│"),
           PipeType::LeftUp => write!(f, "┘"),
           PipeType::LeftDown => write!(f, "┐"),
           PipeType::RightUp => write!(f, "└"),
           PipeType::RightDown => write!(f, "┌"),
       }
    */

    pub fn get_next_side(&self, coming_from: Side) -> Side {
        let pipe_type = self.pipe_type;
        match (coming_from, pipe_type) {
            (Side::Left, PipeType::Horizontal) => Side::Left,
            (Side::Right, PipeType::Horizontal) => Side::Right,
            (Side::Up, PipeType::Vertical) => Side::Up,
            (Side::Down, PipeType::Vertical) => Side::Down,
            (Side::Right, PipeType::LeftUp) => Side::Up,
            (Side::Down, PipeType::LeftUp) => Side::Left,
            (Side::Right, PipeType::LeftDown) => Side::Down,
            (Side::Up, PipeType::LeftDown) => Side::Left,
            (Side::Left, PipeType::RightUp) => Side::Up,
            (Side::Down, PipeType::RightUp) => Side::Right,
            (Side::Left, PipeType::RightDown) => Side::Down,
            (Side::Up, PipeType::RightDown) => Side::Right,
            _ => panic!("Invalid side combination"),
        }
    }

    pub fn try_connect_first(&mut self, pipe: &mut Pipe) {
        let side = self.first_side;
        if pipe.first_side.is_opposite(&side) {
            self.first_side_connected = true;
            pipe.first_side_connected = true;
        } else if pipe.second_side.is_opposite(&side) {
            self.first_side_connected = true;
            pipe.second_side_connected = true;
        }
    }

    pub fn try_connect_second(&mut self, pipe: &mut Pipe) {
        let side = self.second_side;
        if pipe.first_side.is_opposite(&side) {
            self.second_side_connected = true;
            pipe.first_side_connected = true;
        } else if pipe.second_side.is_opposite(&side) {
            self.second_side_connected = true;
            pipe.second_side_connected = true;
        }
    }

    fn gen_type(side: Side, other_side: Side) -> PipeType {
        match (side, other_side) {
            (Side::Up, Side::Down) => PipeType::Vertical,
            (Side::Down, Side::Up) => PipeType::Vertical,
            (Side::Left, Side::Right) => PipeType::Horizontal,
            (Side::Right, Side::Left) => PipeType::Horizontal,
            (Side::Up, Side::Right) => PipeType::RightUp,
            (Side::Right, Side::Up) => PipeType::RightUp,
            (Side::Left, Side::Up) => PipeType::LeftUp,
            (Side::Up, Side::Left) => PipeType::LeftUp,
            (Side::Left, Side::Down) => PipeType::LeftDown,
            (Side::Down, Side::Left) => PipeType::LeftDown,
            (Side::Right, Side::Down) => PipeType::RightDown,
            (Side::Down, Side::Right) => PipeType::RightDown,
            _ => panic!("Invalid side combination"),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.pipe_type {
            PipeType::Horizontal => write!(f, "─"),
            PipeType::Vertical => write!(f, "│"),
            PipeType::LeftUp => write!(f, "┘"),
            PipeType::LeftDown => write!(f, "┐"),
            PipeType::RightUp => write!(f, "└"),
            PipeType::RightDown => write!(f, "┌"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_connect_l_to_j() {
        //L
        let mut l = Pipe::new(Side::Up, Side::Right);
        //J
        let mut j = Pipe::new(Side::Left, Side::Up);
        l.try_connect_first(&mut j);
        l.try_connect_second(&mut j);
        assert_eq!(l.first_side_connected, false);
        assert_eq!(l.second_side_connected, true);
        assert!(j.first_side_connected);
        assert!(!j.second_side_connected);
    }
}
