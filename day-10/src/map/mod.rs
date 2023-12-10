use std::{
    cell::RefCell,
    fmt::Display,
    str::{Chars, FromStr},
};

use self::pipe::{Pipe, Side};

pub mod pipe;

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pipe(Pipe),
    Empty,
    Start,
}

impl From<char> for Piece {
    fn from(c: char) -> Self {
        match c {
            '|' => Piece::Pipe(Pipe::new(Side::Up, Side::Down)),
            '-' => Piece::Pipe(Pipe::new(Side::Left, Side::Right)),
            'L' => Piece::Pipe(Pipe::new(Side::Up, Side::Right)),
            'J' => Piece::Pipe(Pipe::new(Side::Left, Side::Up)),
            '7' => Piece::Pipe(Pipe::new(Side::Left, Side::Down)),
            'F' => Piece::Pipe(Pipe::new(Side::Right, Side::Down)),
            'S' => Piece::Start,
            _ => Piece::Empty,
        }
    }
}

impl FromStr for Piece {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Piece::Pipe(Pipe::new(Side::Up, Side::Down))),
            "-" => Ok(Piece::Pipe(Pipe::new(Side::Left, Side::Right))),
            "L" => Ok(Piece::Pipe(Pipe::new(Side::Up, Side::Right))),
            "J" => Ok(Piece::Pipe(Pipe::new(Side::Left, Side::Up))),
            "7" => Ok(Piece::Pipe(Pipe::new(Side::Left, Side::Down))),
            "F" => Ok(Piece::Pipe(Pipe::new(Side::Right, Side::Down))),
            "S" => Ok(Piece::Start),
            _ => Ok(Piece::Empty),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub map: Vec<Vec<RefCell<Piece>>>,
}

impl Map {
    pub fn new(input: &str) -> Map {
        let mut map = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let piece = Piece::from(c);
                let piece = RefCell::new(piece);
                row.push(piece);
            }
            map.push(row);
        }
        Map { map }
    }

    pub fn connect_pipes(&mut self) {
        for i in 0..self.map.len() {
            let line = &self.map[i];
            for j in 0..line.len() {
                let piece = &line[j];
                let mut piece = piece.borrow_mut();
                match *piece {
                    Piece::Pipe(ref mut pipe) => {
                        if !pipe.first_side_connected {
                            let other_piece = self.get_other_piece(i, j, pipe.first_side);
                            match other_piece {
                                None => {}
                                Some(other_piece) => {
                                    let mut other_piece = other_piece.borrow_mut();
                                    match *other_piece {
                                        Piece::Pipe(ref mut other_pipe) => {
                                            pipe.try_connect_first(other_pipe);
                                        }
                                        Piece::Start => {
                                            pipe.first_side_connected = true;
                                        }
                                        _ => {}
                                    }
                                }
                            };
                        }
                        if !pipe.second_side_connected {
                            let other_piece = self.get_other_piece(i, j, pipe.second_side);
                            match other_piece {
                                None => {}
                                Some(other_piece) => {
                                    let mut other_piece = other_piece.borrow_mut();
                                    match *other_piece {
                                        Piece::Pipe(ref mut other_pipe) => {
                                            pipe.try_connect_second(other_pipe);
                                        }
                                        Piece::Start => {
                                            pipe.second_side_connected = true;
                                        }
                                        _ => {}
                                    }
                                }
                            };
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn prone_unused_pipes(&mut self) {
        for (i, lines) in self.map.iter().enumerate() {
            for (j, piece) in lines.iter().enumerate() {
                self.remove_piece(i, j);
            }
        }
    }

    fn remove_piece(&self, i: usize, j: usize) {
        let piece = &self.map[i][j];
        let mut piece = piece.borrow_mut();
        let pipe = match *piece {
            Piece::Pipe(ref mut pipe) => pipe,
            _ => return,
        };
        if !pipe.first_side_connected || !pipe.second_side_connected {
            *piece = Piece::Empty;
            return;
        }
        let first_side = self.get_other_piece(i, j, pipe.first_side);
        let second_side = self.get_other_piece(i, j, pipe.second_side);
        if pipe.first_side_connected {
            let first_side = first_side.unwrap();

            let first_side_piece = first_side.borrow_mut();
            match *first_side_piece {
                Piece::Empty => {
                    *piece = Piece::Empty;
                }
                _ => {
                    return;
                }
            }

            //At this point we know that this pipe was pruned

            match second_side {
                None => {}
                Some(other_piece) => {
                    let mut other_piece = other_piece.borrow_mut();
                    match *other_piece {
                        Piece::Pipe(_) => {
                            *other_piece = Piece::Empty;
                        }
                        _ => {}
                    }
                }
            }

            return;
        }
        if pipe.second_side_connected {
            let second_side = second_side.unwrap();

            let second_side_piece = second_side.borrow_mut();
            match *second_side_piece {
                Piece::Empty => {
                    *piece = Piece::Empty;
                }
                _ => {
                    return;
                }
            }

            //At this point we know that this pipe was pruned

            match first_side {
                None => {}
                Some(other_piece) => {
                    let mut other_piece = other_piece.borrow_mut();
                    match *other_piece {
                        Piece::Pipe(_) => {
                            *other_piece = Piece::Empty;
                        }
                        _ => {}
                    }
                }
            }

            return;
        }
    }

    fn get_other_piece(&self, i: usize, j: usize, side: Side) -> Option<&RefCell<Piece>> {
        match side {
            Side::Up => {
                if i == 0 {
                    return None;
                }
                let row = &self.map[i - 1];
                return row.get(j);
            }
            Side::Down => {
                if i == self.map.len() - 1 {
                    return None;
                }
                let row = &self.map[i + 1];
                return row.get(j);
            }
            Side::Left => {
                if j == 0 {
                    return None;
                }
                let row = &self.map[i];
                return row.get(j - 1);
            }
            Side::Right => {
                if j == self.map[i].len() - 1 {
                    return None;
                }
                let row = &self.map[i];
                return row.get(j + 1);
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for piece in row {
                let piece = piece.borrow();
                match *piece {
                    Piece::Pipe(pipe) => write!(f, "{}", pipe)?,
                    Piece::Empty => write!(f, ".")?,
                    Piece::Start => write!(f, "S")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
