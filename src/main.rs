use std::env;
use std::fs;
use std::process;

use rbfi::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Insufficient arguments");
        eprintln!("Usage: {} <FILE>", args[0]);
    }

    let filename = &args[1];

    let contents: Vec<char> = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    }).chars().collect();

    let mut env = Program::BFEnv::new();
    let tokens = Lexer::tokenize(&contents);
    let instructions = Parser::parse(&tokens);

    env.run(&instructions);
}
