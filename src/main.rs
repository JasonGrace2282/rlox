use lox::chunk;
use lox::opcode::Operation;
use lox::vm::VM;

fn main() {
    let mut chunk = chunk::Chunk::new();
    chunk.add_constant(52, 1);
    chunk.add_constant(12, 2);
    chunk.write(Operation::Return, 3);
    chunk.add_constant(14, 4);
    let mut vm = VM::new(chunk);
    vm.interpret().expect("yup");
}
