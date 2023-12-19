use std::collections::HashMap;

use crate::{FunctionMap, Piece};

use super::types::{BinaryComparison, Comparator, Function, Result, Return, Statement, Variable};

pub fn lex_pieces(piece: &str) -> Vec<Piece> {
    let mut pieces = Vec::new();
    for line in piece.lines() {
        let mut piece_definition = line.trim_start_matches('{').trim_end_matches('}');
        //x=787,m=2655,a=1222,s=2876
        let mut piece = [0; 4];
        loop {
            let mid = match piece_definition.find(',') {
                Some(i) => i,
                None => {
                    lex_variable(piece_definition, &mut piece);
                    break;
                }
            };
            let (variable, rest2) = piece_definition.split_at(mid);
            let rest2 = rest2.trim_start_matches(',');
            let variable = variable.trim();
            lex_variable(variable, &mut piece);
            piece_definition = rest2;
            if piece_definition.len() == 0 {
                break;
            }
        }
        pieces.push(piece);
    }
    pieces
}

pub fn lex_functions(input: &str) -> FunctionMap {
    let mut map: FunctionMap = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        let (function_name, rest) = line.split_at(line.find('{').unwrap());
        let function_name = function_name.trim();
        let function_definition = rest.trim_start_matches('{').trim_end_matches('}');
        let function = lex_function(function_definition);
        map.insert(function_name.to_string(), function);
    }
    map
}

fn lex_variable(variable: &str, piece: &mut Piece) {
    let mut chars = variable.chars();
    let variable = chars.next().unwrap();
    chars.next();
    let value = chars.as_str().parse::<u16>().unwrap();
    let variable = Variable::from_char(variable);
    piece[variable as usize] = value;
}

fn lex_function(function_definition: &str) -> Function {
    let mut function: Function = Vec::new();
    let (statement, rest) = function_definition.split_at(function_definition.find(',').unwrap());
    let mut rest = rest.trim_start_matches(',');
    let statement = statement.trim();
    let statement = lex_statement(statement);
    function.push(statement);
    loop {
        let mid = match rest.find(',') {
            Some(i) => i,
            None => {
                let statement = lex_statement(rest);
                function.push(statement);
                break;
            }
        };
        let (statement, rest2) = rest.split_at(mid);
        let rest2 = rest2.trim_start_matches(',');
        let statement = statement.trim();
        let statement = lex_statement(statement);
        function.push(statement);
        rest = rest2;
        if rest.len() == 0 {
            break;
        }
    }

    function
}

fn lex_statement(statement: &str) -> Statement {
    let second_char = match statement.chars().nth(1) {
        Some(c) => c,
        None => {
            return lex_return(statement);
        }
    };
    match second_char {
        '<' | '>' | '=' => lex_comparator(statement),
        _ => lex_return(statement),
    }
}

fn lex_comparator(statement: &str) -> Statement {
    let mut chars = statement.chars();
    let variable = match chars.next().unwrap() {
        'a' => Variable::A,
        'm' => Variable::M,
        's' => Variable::S,
        'x' => Variable::X,
        _ => panic!("Invalid variable"),
    };
    let comparison = match chars.next().unwrap() {
        '<' => BinaryComparison::LessThan,
        '>' => BinaryComparison::GreaterThan,
        _ => panic!("Invalid comparison"),
    };
    let (value, retur) = chars.as_str().split_at(chars.as_str().find(':').unwrap());
    let value = value.parse::<u16>().unwrap();
    let retur = retur.trim_start_matches(':');

    let return_value = match retur {
        "A" => Return::Final(Result::Accept),
        "R" => Return::Final(Result::Reject),
        _ => Return::Function(retur.to_string()),
    };
    return Statement::Compare(Comparator {
        variable,
        comparison,
        value,
        return_value,
    });
}

fn lex_return(statement: &str) -> Statement {
    let mut chars = statement.chars();
    let return_value = match chars.next().unwrap() {
        'A' => Return::Final(Result::Accept),
        'R' => Return::Final(Result::Reject),
        _ => Return::Function(statement.to_string()),
    };
    Statement::Return(return_value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let map = lex_functions(input);
        println!("{:?}", map);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_lex2() {
        let input = "lnx{m>1548:A,A}";
        let map = lex_functions(input);
        println!("{:?}", map);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_multiple_functions() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}";
        let map = lex_functions(input);
        assert_eq!(map.len(), 11);
    }
}
