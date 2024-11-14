use std::env;
use std::fs;

use compiler::Compiler;
use compiler::CompilerOptions;
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
    let output_path: String = String::from("C:\\Users\\rxgqq\\projects\\rbf\\example\\out.asm");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Run with: cargo run <path> <mode>");
        return;
    }

    let (file_path, mode) = (args[1].as_str(), map_compiler_mode(args[2].clone()));

    let source = fs::read_to_string(file_path)
        .expect("Error reading file.");

    let mut lexer = Lexer::new(mode, source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_)     => {
            println!("\nFailed to compile application.");
            return;
        }
    };

    let mut parser = Parser::new(mode, tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(_)    => return
    };

    let options = CompilerOptions {
        output_path: output_path
    };
    
    let compiler = Compiler::new(options, ast);
    compiler.compile().unwrap_or_else(|_| {
        eprintln!("Compilation failed.");
        return;
    });

    println!("\nCompilation finished.");
}