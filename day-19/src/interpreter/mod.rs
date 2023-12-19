use crate::{
    compiler::{
        types::{BinaryComparison, Function, Result, Return, Statement},
        Compiled,
    },
    FunctionMap,
};

pub mod range;

pub fn evaluate(compile: Compiled) -> Vec<Result> {
    let mut results = Vec::new();
    let functions = compile.functions;
    let pieces = compile.pieces;
    let main_function = functions.get("in").unwrap();
    for piece in pieces {
        let result = evaluate_piece(piece, &functions, main_function);
        results.push(result);
    }
    return results;
}

fn evaluate_piece(piece: [u16; 4], functions: &FunctionMap, main_function: &Function) -> Result {
    let variables = piece;
    let mut current_function = main_function;
    loop {
        let evaluation = evaluate_function(&variables, current_function);
        match evaluation {
            FunctionEvaluation::Function(function_name) => {
                current_function = functions.get(&function_name).unwrap();
            }
            FunctionEvaluation::Final(result) => {
                return result;
            }
        }
    }
}

enum FunctionEvaluation {
    Function(String),
    Final(Result),
}

fn evaluate_function(piece: &[u16; 4], function: &Function) -> FunctionEvaluation {
    for statement in function {
        match statement {
            Statement::Compare(comparator) => {
                let variable = comparator.variable;
                let comparison = comparator.comparison;
                let value = comparator.value;
                let return_value = comparator.return_value.clone();
                let variable_value = piece[variable as usize];
                let result = match comparison {
                    BinaryComparison::GreaterThan => variable_value > value,
                    BinaryComparison::LessThan => variable_value < value,
                };
                if result {
                    match return_value {
                        Return::Function(function_name) => {
                            return FunctionEvaluation::Function(function_name);
                        }
                        Return::Final(result) => {
                            return FunctionEvaluation::Final(result);
                        }
                    }
                }
            }
            Statement::Return(result) => match result {
                Return::Function(function_name) => {
                    return FunctionEvaluation::Function(function_name.clone());
                }
                Return::Final(result) => {
                    return FunctionEvaluation::Final(*result);
                }
            },
        }
    }
    panic!("No return statement found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = include_str!("../inputs/test.txt");
        let compiled = crate::compiler::compile(input);
        let results = evaluate(compiled);
        assert_eq!(
            results,
            vec![
                Result::Accept,
                Result::Reject,
                Result::Accept,
                Result::Reject,
                Result::Accept
            ]
        );
    }
}
