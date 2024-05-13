use crate::value::Value;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}
