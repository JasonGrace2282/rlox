use crate::chunk::Chunk;
use crate::opcode::Operation;

#[derive(Debug)]
pub enum RuntimeError {
    CompileError,
    RuntimeError
}

pub struct VM {
    code: Chunk,
    ip: u32,
}


impl VM {
    pub fn new(code: Chunk) -> Self
    {
        Self {
            code,
            ip: 0,
        }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError>
    {
        loop {
            if let Some(op) = self.code.get_operation(self.ip as usize) {
                match op {
                    Operation::Return => return Ok(()),
                    Operation::Constant(c) => println!("Constant {c}"),
                };
            } else {
                return Ok(());
            }
            self.ip+=1;
        }
    }
}
