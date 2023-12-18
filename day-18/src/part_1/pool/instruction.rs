#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Instruction {
    pub direction: Direction,
    pub length: usize,
}

impl Instruction {
    pub fn new(direction: Direction, length: usize) -> Self {
        Self { direction, length }
    }

    pub fn new_instructions(input: &str) -> Vec<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let first = line.split(" ").next().unwrap();
            let direction = match first {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let length = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            let instruction = Self::new(direction, length);
            instructions.push(instruction);
        }
        instructions
    }

    pub fn get_cos(&self) -> (isize, isize) {
        let mut i_cos = 0;
        let mut j_cos = 0;
        match self.direction {
            Direction::Right => j_cos = 1,
            Direction::Left => j_cos = -1,
            Direction::Down => i_cos = 1,
            Direction::Up => i_cos = -1,
        };
        (i_cos, j_cos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blueprint {
    pub i_len: usize,
    pub j_len: usize,
    pub i_start: usize,
    pub j_start: usize,
}

impl Blueprint {
    pub fn new(instructions: &Vec<Instruction>) -> Self {
        let mut position = (0, 0);
        let mut min_i = isize::MAX;
        let mut max_i = isize::MIN;

        let mut min_j = isize::MAX;
        let mut max_j = isize::MIN;

        let first_instruction = instructions.first().unwrap();
        let len = first_instruction.length;
        let (i_cos, j_cos) = first_instruction.get_cos();
        let i = i_cos * (len - 1) as isize;
        let j = j_cos * (len - 1) as isize;

        position = (position.0 + i, position.1 + j);

        for instruction in instructions.iter().skip(1) {
            let len = instruction.length;
            let mut i_cos = 0;
            let mut j_cos = 0;
            match instruction.direction {
                Direction::Right => j_cos = 1,
                Direction::Left => j_cos = -1,
                Direction::Down => i_cos = 1,
                Direction::Up => i_cos = -1,
            };

            let i = i_cos * len as isize;
            let j = j_cos * len as isize;

            position = (position.0 + i, position.1 + j);
            if position.0 < min_i {
                min_i = position.0;
            }
            if position.0 > max_i {
                max_i = position.0;
            }
            if position.1 < min_j {
                min_j = position.1;
            }
            if position.1 > max_j {
                max_j = position.1;
            }
        }
        return Self {
            i_len: (max_i + min_i.abs()) as usize + 1,
            j_len: (max_j + min_j.abs()) as usize + 1,
            i_start: min_i.abs() as usize,
            j_start: min_j.abs() as usize,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_instructions() {
        let input = include_str!("../../inputs/test.txt");
        let instructions = Instruction::new_instructions(input);
        let blueprint = Blueprint::new(&instructions);
        assert_eq!(blueprint.i_len, 10);
        assert_eq!(blueprint.j_len, 7);
        assert_eq!(blueprint.i_start, 0);
        assert_eq!(blueprint.j_start, 1);
    }

    #[test]
    fn test_new_instructions2() {
        let input = include_str!("../../inputs/input.txt");
        let instructions = Instruction::new_instructions(input);
        let blueprint = Blueprint::new(&instructions);
        println!("{:?}", blueprint);
    }
}
