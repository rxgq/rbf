use std::env;
use std::fs;

use lexer::Lexer;
use ast::Parser;

mod lexer;
mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("Source code path was not provided");
            return;
        }
    };

    let source = fs::read_to_string(file_path)
        .expect("Error reading file.");

    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    lexer.print();

    let parser = Parser::new(tokens);
    let ast = parser.parse();

}