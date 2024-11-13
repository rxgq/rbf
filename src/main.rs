use std::env;
use std::fs;

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

    let tokens = lexer::tokenize(source);
    for token in &tokens {
        println!("{}", token);
    }

    let ast = ast::parse(tokens);
    // for node in ast {
    //     println!("{:?}", node);
    // }

}