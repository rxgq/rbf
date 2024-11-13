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

pub enum LexerDefect {
    InvalidSyntax(i32, char),
    UnmatchedLoopSymbol(i32),
}

impl Display for LexerDefect {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self {
            Self::InvalidSyntax(line, char) => write!(f, "Invalid syntax character: '{}' on line {}", char, line),
            Self::UnmatchedLoopSymbol(line) => write!(f, "Unmatched loop symbol on line {}", line),
        };
    }
}