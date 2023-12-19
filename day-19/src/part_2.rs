use crate::{compiler, interpreter};

pub fn process(input: &str) -> u64 {
    let functions = compiler::compile_only_functions(input);
    let result = interpreter::range::evaluate(functions);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("./inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 167_409_079_868_000);
    }

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 132_186_256_794_011);
    }
}
