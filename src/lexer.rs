use crate::token::LexerError;
use crate::token::Token;
use crate::token::LexerWarning;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum CompilerMode {
    Debug,
    Normal
}

pub struct Lexer {
    mode: CompilerMode,
    source: String,
    line: u32,
    start_loop_count: u32,
    end_loop_count: u32,
    tokens: Vec<Token>,
    errors: Vec<LexerError>,
    warnings: Vec<LexerWarning>,
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
            errors: Vec::new(),
            warnings: Vec::new()
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, bool> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = self.source.chars().peekable();

        let mut previous_char = None;
        let mut current_line_length = 0;
        let mut line_length_warned = false;

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

            if current_line_length > 63 && !line_length_warned {
                self.warnings.push(LexerWarning::CharacterLineWarning(self.line));
                line_length_warned = true;
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
                    if previous_char == Some('[') {
                        self.warnings.push(LexerWarning::EmptyLoopWarning(self.line));
                    }

                    self.end_loop_count += 1;
                    Token::LoopEnd
                },

                '\r' | ' ' => continue,
                '\n' => {
                    self.line += 1;
                    current_line_length = 0;
                    line_length_warned = false;
                    continue;
                }
                _   => {
                    self.errors.push(LexerError::InvalidSyntax(self.line, char));
                    continue;
                },
            };

            tokens.push(token);
            previous_char = Some(char);
            current_line_length += 1;
        }

        self.tokens = tokens;

        if self.start_loop_count != self.end_loop_count {
            self.errors.push(LexerError::UnmatchedLoopSymbol(self.line));
        }
        
        if self.mode == CompilerMode::Debug {
            self.debug_print();
        }
        
        for defect in &self.warnings {
            println!("{}", defect)
        }
        for error in &self.errors {
            println!("{}", error)
        }

        println!("\nGenerated {} warning(s)", self.warnings.len());
        println!("Generated {} errors(s)", self.errors.len());

        return if self.errors.is_empty() { Ok(self.tokens.clone()) } else { Err(false) };
    }
    
    pub fn debug_print(&self) {
        println!("Tokens:");
        for token in &self.tokens {
            println!("  {}", token);
        }
        println!("");
        
        println!("StartLoop Count: {}", self.start_loop_count);
        println!("EndLoop Count:   {}", self.end_loop_count);
        println!("Lines:           {}", self.line);
        println!("Defects:         {}", self.warnings.len());
        println!("Errors:          {}", self.errors.len());
        println!("");
    }
}