use crate::chunk::Chunk;
use crate::opcode::Operation;
use crate::value::Value;

use std::error::Error;
use std::fmt;

macro_rules! binary_op {
    ($self:ident, $op:tt) => {{
        let top1 = $self.pop();
        let top2 = $self.pop();
        // Do it in reverse so that division
        // and subtraction happen correctly
        $self.push(top2 $op top1);
    }};
}

#[derive(Debug)]
pub enum RuntimeError {
    CompileError,
    InternalError,
    RuntimeError,
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down! {:#?}", self)
    }
}

#[derive(Debug)]
pub struct VM {
    code: Chunk,
    ip: u32,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(code: Chunk) -> Self {
        Self {
            code,
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        loop {
            if let Some(op) = self.code.get_operation(self.ip as usize) {
                println!("======= STACK =======");
                println!("{:#?}", self.stack);
                println!("=====================");
                match op {
                    Operation::Return => return Ok(()),
                    Operation::Constant(c) => self.push(c),
                    Operation::Negate => {
                        let temp = -self.pop();
                        self.push(temp);
                    }
                    Operation::Add => binary_op!(self, +),
                    Operation::Subtract => binary_op!(self, -),
                    Operation::Multiply => binary_op!(self, *),
                    Operation::Divide => binary_op!(self, /),
                };
            } else {
                return Ok(());
            }
            self.ip += 1;
        }
    }

    fn push(&mut self, v: Value) {
        self.stack.push(v);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Empty stack")
    }
}
