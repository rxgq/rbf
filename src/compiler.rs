use std::{fs::File, io::Write};

use crate::ast_node::{ASTNode, AST};

pub struct CompilerOptions {
    pub output_path: String
}


pub struct Compiler {
    options: CompilerOptions,
    ast: AST
}

impl Compiler {
    pub fn new(options: CompilerOptions, ast: AST) -> Compiler {
        Compiler {
            options,
            ast
        }
    }

    pub fn compile(&self) -> std::io::Result<()> {
        let mut output = String::new();

        for node in &self.ast.body {
            match node {
                ASTNode::IncPtrNode => output.push_str("    inc rbx\n"),
                ASTNode::DecPtrNode => output.push_str("    dec rbx\n"),
                ASTNode::IncValNode => output.push_str("    inc byte [rbx]\n"),
                ASTNode::DecValNode => output.push_str("    dec byte [rbx]\n"),
                _ => output.push_str("")
            };
        }

        let mut file = File::create(&self.options.output_path)?;
        file.write_all(output.as_bytes());

        return Ok(());
    }
} 