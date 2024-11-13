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

pub enum LexerWarning {
    EmptyLoopWarning(u32),
    CharacterLineWarning(u32)
}

impl Display for LexerWarning {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self {
            Self::EmptyLoopWarning(line)     => write!(f, "WARNING: Empty loop on line {}", line),
            Self::CharacterLineWarning(line) => write!(f, "WARNING: Line {} is above 32 characters. Try splitting it up to preserve readability", line),
        };
    }
}

pub enum LexerError {
    InvalidSyntax(u32, char),
    UnmatchedLoopSymbol(u32),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self {
            Self::InvalidSyntax(line, char) => write!(f, "ERROR: Invalid syntax character: '{}' on line {}", char, line),
            Self::UnmatchedLoopSymbol(line) => write!(f, "ERROR: Unmatched loop symbol on line {}", line),
        };
    }
}
