use crate::token::Token;
use crate::token::LexerDefect;

#[derive(PartialEq)]
pub enum LexerMode {
    Debug,
    Normal
}

pub struct Lexer {
    mode: LexerMode,
    source: String,
    line: i32,
    tokens: Vec<Token>,
    pub defects: Vec<LexerDefect>
}

impl Lexer {
    pub fn new(mode: LexerMode, source: String) -> Lexer {
        Lexer {
            mode,
            source,
            line: 1,
            tokens: Vec::new(),
            defects: Vec::new()
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        
        let mut chars = self.source.chars().peekable();
    
        while let Some(char) = chars.next() {
            if char == '#' {
                while let Some(&next_char) = chars.peek() {
                    if next_char == '\n' {
                        chars.next();
                        break;
                    }
                    chars.next();
                }
                continue;
            }

            let token = match char {
                '>' => Token::IncPtr,
                '<' => Token::DecPtr,
                '+' => Token::IncVal,
                '-' => Token::DecVal,
                '.' => Token::Output,
                ',' => Token::Input,
                '[' => Token::LoopStart,
                ']' => Token::LoopEnd,

                '\r' | ' ' => continue,
                '\n' => {
                    self.line += 1;
                    continue;
                }
                _   => {
                    self.defects.push(LexerDefect::InvalidSyntax(self.line, char));
                    continue;
                },
            };

            tokens.push(token);
        }

        self.tokens = tokens;

        if self.mode == LexerMode::Debug {
            self.print();
        }

        return self.tokens.clone();
    }
    
    pub fn print(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }
}
