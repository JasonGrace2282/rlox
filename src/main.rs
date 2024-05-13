use std::{
    io::{self, Write},
    env,
    fs,
    process
};

use lox::scanner;

fn main() {
    // let mut chunk = chunk::Chunk::new();
    // chunk.add_constant(52.0, 1);
    // chunk.write(Operation::Negate, 1);
    // chunk.add_constant(0.5, 2);
    // chunk.write(Operation::Multiply, 2);
    // chunk.add_constant(2_f32, 3);
    // chunk.write(Operation::Divide, 3);
    // let mut vm = VM::new(chunk);
    // vm.interpret().expect("yup");
    // dbg!(vm);
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    }
}

fn repl()
{
    println!("rLox v0.1.0");
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => eprintln!("{e}")
        };
        let codeline = input.trim().to_string();
        if codeline == "exit" {
            process::exit(0);
        }
        run_code(codeline).unwrap_or_else(|e| eprintln!("{}", e));
    }
}

fn run_file(file: &String)
{
    match fs::read_to_string(file) {
        Ok(c) => run_code(c).unwrap_or_else(|e| eprintln!("{}", e)),
        Err(e) => {
            eprintln!("An error occured with accessing the file: {}", e);
            process::exit(1);
        }
    };
}

fn run_code(code: String) -> Result<(), &'static str>
{
    scanner::compile(code)
}
