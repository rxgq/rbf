use crate::token::Token;
use crate::token::LexerDefect;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum CompilerMode {
    Debug,
    Normal
}

pub struct Lexer {
    mode: CompilerMode,
    source: String,
    start_loop_count: i32,
    end_loop_count: i32,
    line: i32,
    tokens: Vec<Token>,
    defects: Vec<LexerDefect>
}

impl Lexer {
    pub fn new(mode: CompilerMode, source: String) -> Lexer {
        Lexer {
            mode,
            source,
            line: 1,
            start_loop_count: 0,
            end_loop_count: 0,
            tokens: Vec::new(),
            defects: Vec::new()
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, bool> {
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
                '[' => {
                    self.start_loop_count += 1;
                    Token::LoopStart
                },
                ']' => {
                    self.end_loop_count += 1;
                    Token::LoopEnd
                },

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

        if self.start_loop_count != self.end_loop_count {
            self.defects.push(LexerDefect::UnmatchedLoopSymbol(self.line));
        }
        
        if self.mode == CompilerMode::Debug {
            self.print();
        }

        if !self.defects.is_empty() {
            for defect in &self.defects {
                println!("{}", defect)
            }

            return Err(false)
        }

        return Ok(self.tokens.clone());
    }
    
    pub fn print(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }

        println!("StartLoop Count: {}", self.start_loop_count);
        println!("EndLoop Count:   {}", self.end_loop_count);
        println!("Lines:           {}", self.line);
        println!("Defects:         {}", self.defects.len());
    }
}