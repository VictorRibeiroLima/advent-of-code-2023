use crate::{FunctionMap, Piece};

mod lexer;
pub mod types;

pub struct Compiled {
    pub pieces: Vec<Piece>,
    pub functions: FunctionMap,
}

pub fn compile_only_functions(input: &str) -> FunctionMap {
    let mut blocks = input.split("\n\n");
    let functions = blocks.next().unwrap();
    lexer::lex_functions(functions)
}

pub fn compile(input: &str) -> Compiled {
    let mut blocks = input.split("\n\n");
    let functions = blocks.next().unwrap();
    let pieces = blocks.next().unwrap();
    let functions = lexer::lex_functions(functions);
    let pieces = lexer::lex_pieces(pieces);

    Compiled { pieces, functions }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compile() {
        let input = include_str!("../inputs/test.txt");
        compile(input);
    }
}
