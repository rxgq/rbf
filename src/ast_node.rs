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
            Self::IncPtrNode  => write!(f, "Node(>)"),
            Self::DecPtrNode  => write!(f, "Node(<)"),
            Self::IncValNode  => write!(f, "Node(+)"),
            Self::DecValNode  => write!(f, "Node(-)"),
            Self::OutputNode  => write!(f, "Node(.)"),
            Self::InputNode   => write!(f, "Node(,)"),
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

#[derive(Clone)]
pub struct AST {
    pub body: Vec<ASTNode>,
}

impl AST {
    pub fn new() -> AST {
        AST { 
            body: Vec::new() 
        }
    }
}