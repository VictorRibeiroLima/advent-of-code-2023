use crate::{
    compiler::{self, types::Result},
    interpreter,
};

pub fn process(input: &str) -> usize {
    let mut final_result = 0;
    let compiled = compiler::compile(input);
    let pieces = compiled.pieces.clone();
    let results = interpreter::evaluate(compiled);
    for (result, piece) in results.iter().zip(pieces.iter()) {
        match result {
            Result::Accept => {
                let piece_value = piece.iter().sum::<u16>() as usize;
                final_result += piece_value;
            }
            Result::Reject => {}
        }
    }
    final_result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("./inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 19114);
    }

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 495_298);
    }
}
