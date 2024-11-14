use std::fmt::{self, Display, Formatter};

#[derive(Clone)] 
pub enum ASTNode {
    IncPtrNode,
    DecPtrNode,
    IncValNode,
    DecValNode,
    InputNode,
    OutputNode,
    LoopNode(Vec<ASTNode>),
}

impl ASTNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
        let indent = "  ".repeat(depth);
        match self {
            Self::IncPtrNode => write!(f, "{}Node(>)", indent),
            Self::DecPtrNode => write!(f, "{}Node(<)", indent),
            Self::IncValNode => write!(f, "{}Node(+)", indent),
            Self::DecValNode => write!(f, "{}Node(-)", indent),
            Self::OutputNode => write!(f, "{}Node(.)", indent),
            Self::InputNode  => write!(f, "{}Node(,)", indent),
            Self::LoopNode(nodes) => {
                writeln!(f, "{}Loop(", indent)?;
                for node in nodes {
                    node.fmt_with_indent(f, depth + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{})", indent)
            }
        }
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0) 
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
