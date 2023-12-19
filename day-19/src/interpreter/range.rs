use crate::{
    compiler::types::{BinaryComparison, Result, Return, Statement, Variable},
    FunctionMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PieceRange {
    start: u64,
    end: u64,
}

impl PieceRange {
    fn size(&self) -> u64 {
        let result = self.end - self.start;
        if result == 0 {
            //returning 0 causes the batch entire size to be 0
            return 1;
        }
        result
    }
}

struct Batch {
    x: PieceRange,
    m: PieceRange,
    a: PieceRange,
    s: PieceRange,
}

impl Batch {
    fn size(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }

    fn partial(&self, variable: Variable, range: PieceRange) -> Self {
        match variable {
            Variable::X => Batch {
                x: range,
                m: self.m,
                a: self.a,
                s: self.s,
            },
            Variable::M => Batch {
                x: self.x,
                m: range,
                a: self.a,
                s: self.s,
            },
            Variable::A => Batch {
                x: self.x,
                m: self.m,
                a: range,
                s: self.s,
            },
            Variable::S => Batch {
                x: self.x,
                m: self.m,
                a: self.a,
                s: range,
            },
        }
    }
}

pub fn evaluate(functions: FunctionMap) -> u64 {
    let batch = Batch {
        x: PieceRange {
            start: 1,
            end: 4001,
        },
        m: PieceRange {
            start: 1,
            end: 4001,
        },
        a: PieceRange {
            start: 1,
            end: 4001,
        },
        s: PieceRange {
            start: 1,
            end: 4001,
        },
    };

    evaluate_function(batch, &functions, Return::Function("in".to_string()))
}

fn evaluate_function(batch: Batch, functions: &FunctionMap, last_return: Return) -> u64 {
    let function = match last_return {
        Return::Function(function_name) => functions.get(&function_name).unwrap(),
        Return::Final(result) => match result {
            Result::Accept => {
                let size = batch.size();
                return size;
            }
            Result::Reject => {
                return 0;
            }
        },
    };
    let mut batch = batch;
    let mut result = 0;

    for statement in function {
        match statement {
            Statement::Compare(comparator) => {
                let variable = comparator.variable;
                let comparison = comparator.comparison;
                let value = comparator.value;
                let return_value = comparator.return_value.clone();
                let start = match variable {
                    Variable::X => batch.x.start,
                    Variable::M => batch.m.start,
                    Variable::A => batch.a.start,
                    Variable::S => batch.s.start,
                } as u16;
                let end = match variable {
                    Variable::X => batch.x.end,
                    Variable::M => batch.m.end,
                    Variable::A => batch.a.end,
                    Variable::S => batch.s.end,
                } as u16;
                match comparison {
                    BinaryComparison::GreaterThan => {
                        if start > value {
                            //Full range satisfies
                            result += evaluate_function(batch, functions, return_value);
                            return result;
                        } else if end > value + 1 {
                            let matched_batch = batch.partial(
                                variable,
                                PieceRange {
                                    start: (value + 1) as u64,
                                    end: end as u64,
                                },
                            );
                            //Evaluate the matched batch
                            result += evaluate_function(matched_batch, functions, return_value);
                            //Update the batch to be the unmatched part
                            batch = batch.partial(
                                variable,
                                PieceRange {
                                    start: start as u64,
                                    end: (value + 1) as u64,
                                },
                            );
                        } else {
                            //No match,"removing" range from the equation
                            batch = batch.partial(variable, PieceRange { start: 1, end: 1 });
                        }
                    }
                    BinaryComparison::LessThan => {
                        if end <= value {
                            //Full range satisfies
                            result += evaluate_function(batch, functions, return_value);
                            return result;
                        } else if start < value {
                            let matched_batch = batch.partial(
                                variable,
                                PieceRange {
                                    start: start as u64,
                                    end: value as u64,
                                },
                            );
                            result += evaluate_function(matched_batch, functions, return_value);
                            batch = batch.partial(
                                variable,
                                PieceRange {
                                    start: value as u64,
                                    end: end as u64,
                                },
                            );
                        } else {
                            //No match,"removing" range from the equation
                            batch = batch.partial(variable, PieceRange { start: 1, end: 1 });
                        }
                    }
                };
            }
            Statement::Return(return_value) => match return_value {
                _ => {
                    //Fallthrough
                    result += evaluate_function(batch, functions, return_value.clone());
                    return result;
                }
            },
        }
    }
    panic!("No return statement");
}
