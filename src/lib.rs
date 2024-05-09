#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
}

pub type Value = i32;

struct ValueArray {
    array: Vec<Value>,
    lines: Vec<Value>
}

impl ValueArray {
    pub fn new() -> Self
    {
        Self {
            array: Vec::new(),
            lines: Vec::new()
        }
    }

    pub fn write_value(&mut self, value: Value)
    {
        self.array.push(value);
    }
}


struct Chunk {
    code: String,
    lines: Vec<i32>,
    constants: ValueArray
}

impl Chunk {
    pub fn new() -> Self
    {
        Self {
            code: String::from(""),
            constants: ValueArray::new()
        }
    }

    pub fn write(&mut self, value: &str)
    {
        self.code.push_str(value);
    }

    pub fn add_constant(&mut self, value: Value)
    {
        self.constants.write_value(value);
    }
}
