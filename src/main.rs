use lox::chunk;
use lox::opcode::Operation;
use lox::vm::VM;

fn main() {
    let mut chunk = chunk::Chunk::new();
    chunk.add_constant(52.0, 1);
    chunk.write(Operation::Negate, 1);
    chunk.add_constant(0.5, 2);
    chunk.write(Operation::Multiply, 2);
    chunk.add_constant(2_f32, 3);
    chunk.write(Operation::Divide, 3);
    let mut vm = VM::new(chunk);
    vm.interpret().expect("yup");
    dbg!(vm);
}
