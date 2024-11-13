use core::panic;

use crate::{ast_node::ASTNode, lexer::CompilerMode, token::Token};


pub struct Parser {
    mode: CompilerMode,
    tokens: Vec<Token>,
    nodes: Vec<ASTNode>
}

impl Parser {
    pub fn new(mode: CompilerMode, tokens: Vec<Token>) -> Parser {
        Parser { 
            mode,
            tokens,
            nodes: Vec::new()
        }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut idx = 0;

        while idx < self.tokens.len() {
            match self.tokens[idx] {
                Token::IncPtr    => self.nodes.push(ASTNode::IncPtrNode),
                Token::DecPtr    => self.nodes.push(ASTNode::DecPtrNode),
                Token::IncVal    => self.nodes.push(ASTNode::IncValNode),
                Token::DecVal    => self.nodes.push(ASTNode::DecValNode),
                Token::Output    => self.nodes.push(ASTNode::OutputNode),
                Token::Input     => self.nodes.push(ASTNode::InputNode),
                Token::LoopStart => {
                    idx += 1;

                    let mut loop_tokens: Vec<Token> = Vec::new();
                    let mut loop_level = 1;

                    while idx < self.tokens.len() && loop_level > 0 {
                        match self.tokens[idx] {
                            Token::LoopStart => loop_level += 1,
                            Token::LoopEnd => loop_level -= 1,
                            token => loop_tokens.push(token),
                        };
                        idx += 1;
                    }

                    let mut loop_parser = Parser::new(self.mode, loop_tokens);
                    self.nodes.push(ASTNode::Loop(loop_parser.parse()))
                }
                Token::LoopEnd => panic!("Unexpected Loop end symbol."),
            };

            idx += 1
        }

        if self.mode == CompilerMode::Debug {
            self.print();
        }

        return self.nodes.clone();
    }

    pub fn print(&mut self) {
        for node in &self.nodes {
            println!("{}", node)
        }
    }
}