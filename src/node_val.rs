use crate::{command::CommandStrategy, node::Node};

pub enum NodeVal {
    Char(char),
    Num(i64),
    Text(String),
    Boolean(bool),
    Command(Box<dyn CommandStrategy>),
    Null,
}

impl Clone for NodeVal {
    fn clone(&self) -> Self {
        match self {
            NodeVal::Char(c) => NodeVal::Char(*c),
            NodeVal::Num(n) => NodeVal::Num(*n),
            NodeVal::Text(s) => NodeVal::Text(s.clone()),
            NodeVal::Boolean(b) => NodeVal::Boolean(*b),
            NodeVal::Command(_) => panic!("Cannot clone Command variant"),
            NodeVal::Null => NodeVal::Null,
        }
    }
}

impl PartialEq for NodeVal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeVal::Char(a), NodeVal::Char(b)) => a == b,
            (NodeVal::Num(a), NodeVal::Num(b)) => a == b,
            (NodeVal::Text(a), NodeVal::Text(b)) => a == b,
            (NodeVal::Boolean(a), NodeVal::Boolean(b)) => a == b,
            (NodeVal::Command(_), NodeVal::Command(_)) => false,
            (NodeVal::Null, NodeVal::Null) => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for NodeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeVal::Char(c) => write!(f, "Char({})", c),
            NodeVal::Num(n) => write!(f, "Num({})", n),
            NodeVal::Text(s) => write!(f, "Text({})", s),
            NodeVal::Boolean(b) => write!(f, "Boolean({})", b),
            NodeVal::Command(s) => write!(f, "Command({})", s.name()),
            NodeVal::Null => write!(f, "Null"),
        }
    }
}

impl std::fmt::Display for NodeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeVal::Char(c) => write!(f, "'{}'", c),
            NodeVal::Num(n) => write!(f, "{}", n),
            NodeVal::Text(s) => write!(f, "\"{}\"", s),
            NodeVal::Boolean(b) => write!(f, "{}", b),
            NodeVal::Command(s) => write!(f, "<{}>", s.name()),
            NodeVal::Null => write!(f, "<0>"),
        }
    }
}
