use std::collections::HashMap;

use compiler::types::Function;

mod compiler;
mod interpreter;

mod part_1;
mod part_2;

pub type FunctionMap = HashMap<String, Function>;
pub type Piece = [u16; 4];

fn main() {
    let input = include_str!("./inputs/input.txt");
    let part1 = part_1::process(input);
    println!("Part 1: {}", part1);
    let part2 = part_2::process(input);
    println!("Part 2: {}", part2);
}
