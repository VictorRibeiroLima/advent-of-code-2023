use std::{cell::RefCell, fmt::Display, str::FromStr};

use self::pipe::{Pipe, Side};

pub mod pipe;

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pipe(Pipe),
    Empty,
    Start(Option<Side>, Option<Side>),
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PieceType::Pipe(pipe) => {
                if pipe.is_start {
                    write!(f, "\x1b[31m{}\x1b[0m", pipe)
                } else if pipe.main_loop_count.is_some() {
                    write!(f, "\x1b[93m{}\x1b[0m", pipe)
                } else {
                    write!(f, "{}", pipe)
                }
            }
            PieceType::Empty => write!(f, "."),
            PieceType::Start(_, _) => write!(f, "S"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PipeRef {
    pub pipe: Pipe,
    pub point: Point,
    pub next_side: Side,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub i: usize,
    pub j: usize,
}

impl From<char> for PieceType {
    fn from(c: char) -> Self {
        match c {
            '|' => PieceType::Pipe(Pipe::new(Side::Up, Side::Down)),
            '-' => PieceType::Pipe(Pipe::new(Side::Left, Side::Right)),
            'L' => PieceType::Pipe(Pipe::new(Side::Up, Side::Right)),
            'J' => PieceType::Pipe(Pipe::new(Side::Left, Side::Up)),
            '7' => PieceType::Pipe(Pipe::new(Side::Left, Side::Down)),
            'F' => PieceType::Pipe(Pipe::new(Side::Right, Side::Down)),
            'S' => PieceType::Start(None, None),
            _ => PieceType::Empty,
        }
    }
}

impl FromStr for PieceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(PieceType::Pipe(Pipe::new(Side::Up, Side::Down))),
            "-" => Ok(PieceType::Pipe(Pipe::new(Side::Left, Side::Right))),
            "L" => Ok(PieceType::Pipe(Pipe::new(Side::Up, Side::Right))),
            "J" => Ok(PieceType::Pipe(Pipe::new(Side::Left, Side::Up))),
            "7" => Ok(PieceType::Pipe(Pipe::new(Side::Left, Side::Down))),
            "F" => Ok(PieceType::Pipe(Pipe::new(Side::Right, Side::Down))),
            "S" => Ok(PieceType::Start(None, None)),
            _ => Ok(PieceType::Empty),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub map: Vec<Vec<RefCell<PieceType>>>,
    pub polygon_points: Vec<Point>,
}

impl Map {
    pub fn init(input: &str) -> Map {
        let mut map = Map::new(input);
        map.connect_pipes();
        map.prone_unused_pipes();
        map.map_main_loop();
        map.mark_main_loop();
        map
    }

    fn new(input: &str) -> Map {
        let mut map = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let piece = PieceType::from(c);
                let piece = RefCell::new(piece);
                row.push(piece);
            }
            map.push(row);
        }
        Map {
            map,
            polygon_points: Vec::new(),
        }
    }

    fn connect_pipes(&mut self) {
        let mut start_location = (0, 0);
        for i in 0..self.map.len() {
            let line = &self.map[i];
            for j in 0..line.len() {
                let piece = &line[j];
                let mut piece = piece.borrow_mut();
                match *piece {
                    PieceType::Pipe(ref mut pipe) => {
                        if !pipe.first_side_connected {
                            let other_piece = self.get_other_piece(i, j, pipe.first_side);
                            match other_piece {
                                None => {}
                                Some(other_piece) => {
                                    let mut other_piece = other_piece.borrow_mut();
                                    match *other_piece {
                                        PieceType::Pipe(ref mut other_pipe) => {
                                            pipe.try_connect_first(other_pipe);
                                        }
                                        PieceType::Start(None, None) => {
                                            *other_piece =
                                                PieceType::Start(Some(pipe.first_side), None);
                                        }
                                        PieceType::Start(Some(p), None) => {
                                            *other_piece =
                                                PieceType::Start(Some(p), Some(pipe.first_side));
                                        }
                                        PieceType::Start(None, Some(p)) => {
                                            *other_piece =
                                                PieceType::Start(Some(pipe.first_side), Some(p));
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
                                        PieceType::Pipe(ref mut other_pipe) => {
                                            pipe.try_connect_second(other_pipe);
                                        }
                                        PieceType::Start(None, None) => {
                                            *other_piece =
                                                PieceType::Start(Some(pipe.second_side), None);
                                        }
                                        PieceType::Start(Some(p), None) => {
                                            *other_piece =
                                                PieceType::Start(Some(p), Some(pipe.second_side));
                                        }
                                        PieceType::Start(None, Some(p)) => {
                                            *other_piece =
                                                PieceType::Start(Some(pipe.second_side), Some(p));
                                        }
                                        _ => {}
                                    }
                                }
                            };
                        }
                    }
                    PieceType::Start(_, _) => {
                        start_location = (i, j);
                    }
                    _ => (),
                }
            }
        }
        let start_piece = &self.map[start_location.0][start_location.1];
        let mut start_piece = start_piece.borrow_mut();
        match *start_piece {
            PieceType::Start(ref mut first, ref mut second) => {
                let first = first.unwrap();
                let second = second.unwrap();
                let first = first.opposite();
                let second = second.opposite();
                let mut start_pipe = Pipe::new(first, second);
                start_pipe.is_start = true;
                let first = start_pipe.get_clockwise_side();
                let second = start_pipe.get_counterclockwise_side();
                let first_side = self.get_other_piece(start_location.0, start_location.1, first);
                let first_side = first_side.unwrap();
                let mut first_side = first_side.borrow_mut();
                match *first_side {
                    PieceType::Pipe(ref mut pipe) => {
                        start_pipe.try_connect(pipe);
                    }
                    _ => {}
                }
                let second_side = self.get_other_piece(start_location.0, start_location.1, second);
                let second_side = second_side.unwrap();
                let mut second_side = second_side.borrow_mut();
                match *second_side {
                    PieceType::Pipe(ref mut pipe) => {
                        start_pipe.try_connect(pipe);
                    }
                    _ => {}
                }
                let start = PieceType::Pipe(start_pipe);
                *start_piece = start;
            }
            _ => {}
        }
        self.polygon_points.push(Point {
            i: start_location.0,
            j: start_location.1,
        });
    }

    // inefficient, but it works
    fn prone_unused_pipes(&mut self) {
        for (i, lines) in self.map.iter().enumerate() {
            for (j, _) in lines.iter().enumerate() {
                self.remove_piece(i, j);
            }
        }

        loop {
            let mut piece_removed = false;
            for (i, lines) in self.map.iter().enumerate() {
                for (j, _) in lines.iter().enumerate() {
                    let removed = self.remove_piece(i, j);
                    if removed {
                        piece_removed = true;
                    }
                }
            }
            if !piece_removed {
                break;
            }
        }
    }

    fn map_main_loop(&mut self) {
        let first_pipe = self.get_start_piece();
        let mut pipe = first_pipe;
        loop {
            let last_side = pipe.next_side;
            pipe = self.next_pipe(pipe, last_side);
            if pipe.pipe.is_start {
                break;
            }
            self.polygon_points.push(pipe.point);
        }
    }

    fn mark_main_loop(&mut self) {
        for (count, point) in self.polygon_points.iter().enumerate() {
            let i = point.i;
            let j = point.j;
            let piece = &self.map[i][j];
            let mut piece = piece.borrow_mut();
            match *piece {
                PieceType::Pipe(ref mut pipe) => {
                    pipe.main_loop_count = Some(count);
                }
                _ => {}
            }
        }
    }

    fn get_start_piece(&self) -> PipeRef {
        let point = self.polygon_points[0];
        let i = point.i;
        let j = point.j;
        let piece = &self.map[i][j];
        let piece = piece.borrow();
        let pipe = match *piece {
            PieceType::Pipe(pipe) => pipe,
            _ => panic!("Start piece is not a pipe"),
        };
        let side = pipe.get_clockwise_side();
        PipeRef {
            pipe,
            point,
            next_side: side,
        }
    }

    fn next_pipe(&self, pipe_ref: PipeRef, last_side: Side) -> PipeRef {
        let point = pipe_ref.point;
        let original_i = point.i;
        let original_j: usize = point.j;

        let location = self.get_other_piece_location(original_i, original_j, last_side);
        match location {
            None => panic!("No next pipe"),
            Some(point) => {
                let piece = &self.map[point.i][point.j];
                let piece = piece.borrow();

                match *piece {
                    PieceType::Pipe(pipe) => {
                        let side = pipe.get_next_side(last_side);
                        PipeRef {
                            pipe,
                            point,
                            next_side: side,
                        }
                    }
                    _ => panic!("Next piece is not a pipe"),
                }
            }
        }
    }

    fn get_other_piece(&self, i: usize, j: usize, side: Side) -> Option<&RefCell<PieceType>> {
        let location = self.get_other_piece_location(i, j, side);
        match location {
            None => None,
            Some(point) => Some(&self.map[point.i][point.j]),
        }
    }

    fn remove_piece(&self, i: usize, j: usize) -> bool {
        let piece = &self.map[i][j];
        let mut piece = piece.borrow_mut();
        let pipe = match *piece {
            PieceType::Pipe(ref mut pipe) => pipe,
            _ => return false,
        };
        if !pipe.first_side_connected || !pipe.second_side_connected {
            *piece = PieceType::Empty;
            return false;
        }

        if pipe.first_side_connected {
            let first_side = self.get_other_piece(i, j, pipe.first_side);
            let first_side = first_side.unwrap();

            let first_side_piece = first_side.borrow();
            match *first_side_piece {
                PieceType::Empty => {
                    *piece = PieceType::Empty;
                    return true;
                }
                _ => {}
            }
        }
        if pipe.second_side_connected {
            let second_side = self.get_other_piece(i, j, pipe.second_side).unwrap();

            let second_side_piece = second_side.borrow();
            match *second_side_piece {
                PieceType::Empty => {
                    *piece = PieceType::Empty;
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }

    fn get_other_piece_location(&self, i: usize, j: usize, side: Side) -> Option<Point> {
        match side {
            Side::Up => {
                if i == 0 {
                    return None;
                }
                return Some(Point { i: i - 1, j });
            }
            Side::Down => {
                if i == self.map.len() - 1 {
                    return None;
                }
                return Some(Point { i: i + 1, j });
            }
            Side::Left => {
                if j == 0 {
                    return None;
                }
                return Some(Point { i, j: j - 1 });
            }
            Side::Right => {
                if j == self.map[i].len() - 1 {
                    return None;
                }
                return Some(Point { i, j: j + 1 });
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for piece in row {
                let piece = piece.borrow();
                write!(f, "{}", piece)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
