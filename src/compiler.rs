use crate::ast_node::AST;

pub struct CompilerOptions {
    
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

    pub fn compile(&self) {
        for node in &self.ast.body {

        }
    }
} 