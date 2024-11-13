use crate::{ast_node::{ASTNode, AST}, lexer::CompilerMode, token::Token};

#[derive(Clone)]
pub struct Parser {
    program: AST,
    mode: CompilerMode,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(mode: CompilerMode, tokens: Vec<Token>) -> Parser {
        Parser {
            program: AST::new(),
            mode,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<AST, bool> {
        let mut nodes_stack: Vec<Vec<ASTNode>> = vec![Vec::new()];

        for token in &self.tokens {
            match token {
                Token::IncPtr    => nodes_stack.last_mut().unwrap().push(ASTNode::IncPtrNode),
                Token::DecPtr    => nodes_stack.last_mut().unwrap().push(ASTNode::DecPtrNode),
                Token::IncVal    => nodes_stack.last_mut().unwrap().push(ASTNode::IncValNode),
                Token::DecVal    => nodes_stack.last_mut().unwrap().push(ASTNode::DecValNode),
                Token::Output    => nodes_stack.last_mut().unwrap().push(ASTNode::OutputNode),
                Token::Input     => nodes_stack.last_mut().unwrap().push(ASTNode::InputNode),
                Token::LoopStart => {
                    nodes_stack.push(Vec::new());
                },
                Token::LoopEnd => {
                    let loop_nodes = nodes_stack.pop().unwrap();
                    let loop_node = ASTNode::Loop(loop_nodes);
                    nodes_stack.last_mut().unwrap().push(loop_node);
                },
            }
        }

        self.program.body = nodes_stack.pop().unwrap();

        if self.mode == CompilerMode::Debug {
            self.print();
        }

        Ok(self.program.clone())
    }

    pub fn print(&mut self) {
        println!("\nNodes:");
        for node in &self.program.body {
            println!("{}", node)
        }
    }
}