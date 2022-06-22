use std::env;
use std::fs;
use std::process;

use rbfi::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Insufficient arguments");
        eprintln!("Usage: {} <FILE>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    let contents: Vec<char> = fs::read_to_string(filename)
        .unwrap_or_else(|err| {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        })
        .chars()
        .collect();

    let mut env = program::BFEnv::new();
    let tokens = lexer::tokenize(&contents);
    let instructions = parser::parse(&tokens);

    env.run(&instructions);
    println!()
}
