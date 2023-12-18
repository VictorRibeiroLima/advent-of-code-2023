use std::fmt::Display;

use self::instruction::{Blueprint, Instruction};

mod instruction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    Edge(usize),
    Digged,
    Ground,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub node_type: NodeType,
}
impl Node {
    pub fn new() -> Self {
        Self {
            node_type: NodeType::Ground,
        }
    }
}

pub struct Pool {
    pub instructions: Vec<Instruction>,
    pub pool: Vec<Vec<Node>>,
    pub start: (usize, usize),
}

impl Pool {
    pub fn new(input: &str) -> Self {
        let instructions = Instruction::new_instructions(input);
        let blueprint = Blueprint::new(&instructions);
        let j_vec = vec![Node::new(); blueprint.j_len];
        let mut pool = vec![j_vec; blueprint.i_len];
        let mut edge_count = 1;

        let mut position = (blueprint.i_start, blueprint.j_start);

        let first_instruction = instructions.first().unwrap();
        let len = first_instruction.length;
        let (i_cos, j_cos) = first_instruction.get_cos();
        let mut i = position.0;
        let mut j = position.1;
        for _ in 0..len {
            position = (i, j);
            pool[position.0][position.1] = Node {
                node_type: NodeType::Edge(edge_count),
            };
            edge_count += 1;
            i = (position.0 as isize + i_cos) as usize;
            j = (position.1 as isize + j_cos) as usize;
        }

        for instruction in instructions.iter().skip(1) {
            let len = instruction.length;
            let (i_cos, j_cos) = instruction.get_cos();

            let mut i = (position.0 as isize + i_cos) as usize;
            let mut j = (position.1 as isize + j_cos) as usize;
            for _ in 0..len {
                position = (i, j);
                pool[position.0][position.1] = Node {
                    node_type: NodeType::Edge(edge_count),
                };
                edge_count += 1;
                i = (position.0 as isize + i_cos) as usize;
                j = (position.1 as isize + j_cos) as usize;
            }
        }

        Self {
            instructions,
            pool,
            start: (blueprint.i_start, blueprint.j_start),
        }
    }

    pub fn dig(&mut self) {
        for i in 0..self.pool.len() {
            for j in 0..self.pool[i].len() {
                let node = &self.pool[i][j];
                match node.node_type {
                    NodeType::Edge(_) => continue,
                    _ => {}
                }
                if self.check_if_inside((i, j)) {
                    self.pool[i][j].node_type = NodeType::Digged;
                }
            }
        }
    }

    pub fn count_digged(&self) -> usize {
        let mut count = 0;
        for i in 0..self.pool.len() {
            for j in 0..self.pool[i].len() {
                let node = &self.pool[i][j];
                match node.node_type {
                    NodeType::Digged | NodeType::Edge(_) => count += 1,
                    _ => {}
                }
            }
        }
        count
    }

    fn check_if_inside(&self, position: (usize, usize)) -> bool {
        let i = position.0;
        let j = position.1;

        let mut count = 0;

        let j_len = self.pool[0].len();
        for j in j + 1..j_len {
            let edge_count = self.get_position_edge_count((i, j));
            if edge_count.is_none() {
                continue;
            }
            let edge_count = edge_count.unwrap();
            if i > 0 {
                let up_edge_count = self.get_position_edge_count((i - 1, j));
                if up_edge_count.is_some() {
                    let up_edge_count = up_edge_count.unwrap();
                    let diff = (edge_count as isize - up_edge_count as isize).abs();
                    if diff == 1 {
                        if edge_count > up_edge_count {
                            //Descend edge
                            count -= 1;
                        } else {
                            //Ascend edge
                            count += 1;
                        }
                    }
                }
            } else {
                let down_edge_count = self.get_position_edge_count((i + 1, j));
                if down_edge_count.is_some() {
                    let down_edge_count = down_edge_count.unwrap();
                    let diff = (edge_count as isize - down_edge_count as isize).abs();
                    if diff == 1 {
                        if edge_count > down_edge_count {
                            //Descend edge
                            count -= 1;
                        } else {
                            //Ascend edge
                            count += 1;
                        }
                    }
                }
            }
        }

        count != 0
    }

    fn get_position_edge_count(&self, position: (usize, usize)) -> Option<usize> {
        let i = position.0;
        let j = position.1;

        let node = &self.pool[i][j];
        match node.node_type {
            NodeType::Digged => None,
            NodeType::Ground => None,
            NodeType::Edge(edge_count) => Some(edge_count),
        }
    }
}

impl Display for Pool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for i in 0..self.pool.len() {
            for j in 0..self.pool[i].len() {
                let node = &self.pool[i][j];
                match node.node_type {
                    NodeType::Ground => output.push_str("."),
                    _ => output.push_str("#"),
                }
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_pool() {
        let input = include_str!("../../inputs/test.txt");
        let pool = Pool::new(input);
        println!("{}", pool);
    }

    #[test]
    fn test_dig() {
        let input = include_str!("../../inputs/test.txt");

        let mut pool = Pool::new(input);
        println!("{}", pool);
        pool.dig();
        let digged_count = pool.count_digged();
        assert_eq!(digged_count, 62);
    }
}
