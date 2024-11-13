use std::fmt::{self, Display, Formatter};

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
            Self::IncPtr    => write!(f, "IncPtr"),
            Self::DecPtr    => write!(f, "DecPtr"),
            Self::IncVal    => write!(f, "IncVal"),
            Self::DecVal    => write!(f, "DecVal"),
            Self::Output    => write!(f, "Output"),
            Self::Input     => write!(f, "Input"),
            Self::LoopStart => write!(f, "LoopStart"),
            Self::LoopEnd   => write!(f, "LoopEnd"),
        };
    }
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    
    for c in source.chars() {
        let token = match c {
            '>' => Token::IncPtr,
            '<' => Token::DecPtr,
            '+' => Token::IncVal,
            '-' => Token::DecVal,
            '.' => Token::Output,
            ',' => Token::Input,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            _ => todo!()
        };

        tokens.push(token);
    }

    return tokens;
}