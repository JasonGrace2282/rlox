use crate::opcode::{Operation, Value};
use std::default::Default;

#[derive(Debug)]
pub struct ValueArray {
    array: Vec<Operation>,
}


impl Default for ValueArray {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueArray {
    pub fn new() -> Self
    {
        Self {
            array: Vec::new(),
        }
    }

    pub fn write_value(&mut self, op: Operation)
    {
        self.array.push(op);
    }
}


#[derive(Debug)]
pub struct Chunk {
    code: Vec<Operation>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self
    {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, op: Operation, line: u32) {
        self.code.push(op);
        self.lines.push(line);
    }

    /// Adds a constant, and returns an integer pointing to the index
    /// of the constant in the Vector constants
    pub fn add_constant(&mut self, value: Value, line: u32) -> usize
    {
        self.constants.push(value);
        self.write(Operation::Constant(value), line);
        self.constants.len() - 1
    }

    pub fn get_operation(&self, line: usize) -> Option<Operation>
    {
        // self.code: Vec<Operation>
        self.code.get(line).copied()
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
}
