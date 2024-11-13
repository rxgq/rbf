use core::panic;
use crate::lexer::Token;

pub enum ASTNode {
    IncPtrNode,
    DecPtrNode,
    IncValNode,
    DecValNode,
    InputNode,
    OutputNode,
    Loop(Vec<ASTNode>),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&self) -> Vec<ASTNode> {
        let mut nodes: Vec<ASTNode> = Vec::new();
        let mut idx = 0;

        while idx < self.tokens.len() {
            match self.tokens[idx] {
                Token::IncPtr => nodes.push(ASTNode::IncPtrNode),
                Token::DecPtr => nodes.push(ASTNode::DecPtrNode),
                Token::IncVal => nodes.push(ASTNode::IncValNode),
                Token::DecVal => nodes.push(ASTNode::DecValNode),
                Token::Output => nodes.push(ASTNode::OutputNode),
                Token::Input => nodes.push(ASTNode::InputNode),
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

                    let loop_parser = Parser::new(loop_tokens);
                    nodes.push(ASTNode::Loop(loop_parser.parse()));
                }
                Token::LoopEnd => panic!("Unexpected Loop end symbol."),
            };

            idx += 1;
        }

        nodes
    }

    pub fn print(&self) {
        
    }
}
