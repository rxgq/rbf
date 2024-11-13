use std::env;
use std::fs;

use lexer::Lexer;
use parser::Parser;
use utils::map_compiler_mode;

mod lexer;
mod parser;
mod token;
mod compiler;
mod utils;
mod ast_node;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Run with: cargo run <path> <mode>");
        return;
    }

    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("Source code path was not provided");
            return;
        }
    };

    let mode = match args.get(2) {
        Some(mode) => map_compiler_mode(mode.clone()),
        None => {
            eprintln!("Compiler mode not provided");
            return;
        }
    };

    let source = fs::read_to_string(file_path)
        .expect("Error reading file.");

    let mut lexer = Lexer::new(mode, source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_)     => return
    };

    let mut parser = Parser::new(mode, tokens);
    parser.parse();
    
}