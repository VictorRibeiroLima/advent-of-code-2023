use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
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
}

impl Pipe {
    pub fn new(first_side: Side, second_side: Side) -> Pipe {
        Pipe {
            first_side,
            second_side,
            first_side_connected: false,
            second_side_connected: false,
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
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.first_side, &self.second_side) {
            (Side::Up, Side::Down) => write!(f, "|"),
            (Side::Left, Side::Right) => write!(f, "-"),
            (Side::Up, Side::Right) => write!(f, "L"),
            (Side::Left, Side::Up) => write!(f, "J"),
            (Side::Left, Side::Down) => write!(f, "7"),
            (Side::Right, Side::Down) => write!(f, "F"),
            _ => write!(f, " "),
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
