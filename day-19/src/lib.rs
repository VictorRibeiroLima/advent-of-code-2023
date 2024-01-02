use std::collections::HashMap;

use compiler::types::Function;

type FunctionMap = HashMap<String, Function>;
type Piece = [u16; 4];

mod compiler;
mod interpreter;

pub mod part_1;
pub mod part_2;
