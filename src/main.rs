use std::env;
use std::fs;

use lexer::Lexer;
use lexer::LexerMode;

use parser::Parser;
use parser::ParserMode;

mod lexer;
mod parser;
mod token;
mod compiler;

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

    let mut lexer = Lexer::new(LexerMode::Debug, source);
    let tokens = lexer.tokenize();

    if !lexer.defects.is_empty() {
        for defect in lexer.defects {
            println!("{}", defect)
        }
    }

    let parser = Parser::new(ParserMode::Debug, tokens);
    parser.parse();
    
}