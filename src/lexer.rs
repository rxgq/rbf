use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone)]
pub enum Token {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Output,
    Input,
    LoopStart,
    LoopEnd
}


impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self {
            Self::IncPtr    => write!(f, "IncPtr    '>'"),
            Self::DecPtr    => write!(f, "DecPtr    '<'"),
            Self::IncVal    => write!(f, "IncVal    '+'"),
            Self::DecVal    => write!(f, "DecVal    '-'"),
            Self::Output    => write!(f, "Output    '.'"),
            Self::Input     => write!(f, "Input     ','"),
            Self::LoopStart => write!(f, "LoopStart '['"),
            Self::LoopEnd   => write!(f, "LoopEnd   ']'"),
        };
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&self) -> Vec<Token> {
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
                _ => continue,
            };
    
            tokens.push(token);
        }
    
        
    
        return tokens;
    }
    
    pub fn print(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }
}
