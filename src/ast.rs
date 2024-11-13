use core::panic;

use crate::lexer::Token;

pub enum ASTNode {
    IncPtrNode,
    DecPtrNode,
    IncValNode,
    DecValNode,
    InputNode,
    OutputNode,
    Loop(Vec<ASTNode>)
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut nodes: Vec<ASTNode> = Vec::new();
    let mut idx = 0;

    while idx < tokens.len() {
        match tokens[idx] {
            Token::IncPtr    => nodes.push(ASTNode::IncPtrNode),
            Token::DecPtr    => nodes.push(ASTNode::DecPtrNode),
            Token::IncVal    => nodes.push(ASTNode::IncValNode),
            Token::DecVal    => nodes.push(ASTNode::DecValNode),
            Token::Output    => nodes.push(ASTNode::InputNode),
            Token::Input     => nodes.push(ASTNode::OutputNode),
            Token::LoopStart => {
                idx += 1;

                let mut loop_tokens: Vec<Token> = Vec::new();
                let mut loop_level = 1;

                while idx < tokens.len() && loop_level > 0 {
                    match tokens[idx] {
                        Token::LoopStart => loop_level += 1,
                        Token::LoopEnd   => loop_level -= 1,
                        _ => loop_tokens.push(tokens[idx]),
                    };
                    
                    idx += 1;
                }

                nodes.push(ASTNode::Loop(parse(loop_tokens)));
            },
            Token::LoopEnd   => panic!("Unexpected Loop end symbol."),
        };

        idx += 1;
    }

    return nodes;
}