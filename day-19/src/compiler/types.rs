#[derive(Debug, Clone, Copy)]
pub enum Variable {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl Variable {
    pub fn from_char(c: char) -> Variable {
        match c {
            'a' => Variable::A,
            'm' => Variable::M,
            's' => Variable::S,
            'x' => Variable::X,
            _ => panic!("Invalid variable"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryComparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Result {
    Accept,
    Reject,
}

#[derive(Debug, Clone)]
pub enum Return {
    Function(String),
    Final(Result),
}

#[derive(Debug, Clone)]
pub struct Comparator {
    pub variable: Variable,
    pub comparison: BinaryComparison,
    pub value: u16,
    pub return_value: Return,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Compare(Comparator),
    Return(Return),
}

pub type Function = Vec<Statement>;
