use std::fmt::{self, Display, Formatter};

#[derive(Clone)] 
pub enum ASTNode {
    IncPtrNode,
    DecPtrNode,
    IncValNode,
    DecValNode,
    InputNode,
    OutputNode,
    Loop(Vec<ASTNode>),
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncPtrNode  => write!(f, "IncPtrNode"),
            Self::DecPtrNode  => write!(f, "DecPtrNode"),
            Self::IncValNode  => write!(f, "IncValNode"),
            Self::DecValNode  => write!(f, "DecValNode"),
            Self::InputNode   => write!(f, "InputNode"),
            Self::OutputNode  => write!(f, "OutputNode"),
            Self::Loop(nodes) => {
                write!(f, "Loop(")?;

                for (i, node) in nodes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", node)?;
                }
                write!(f, ")")
            }
        }
    }
}
