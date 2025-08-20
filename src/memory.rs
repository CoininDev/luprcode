use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

use crate::node_val::NodeVal;
pub struct Memory {
    pub map: HashMap<MemNode, MemNode>,
}

lazy_static! {
    pub static ref MEMORY: Mutex<Memory> = Mutex::new(Memory {
        map: HashMap::new()
    });
}

#[derive(Hash, PartialEq, Eq)]
pub enum MemNode {
    Char(char),
    Num(i64),
    Text(String),
    Boolean(bool),
    Null,
}

impl From<NodeVal> for MemNode {
    fn from(value: NodeVal) -> Self {
        match value {
            NodeVal::Char(c) => MemNode::Char(c),
            NodeVal::Num(n) => MemNode::Num(n),
            NodeVal::Text(s) => MemNode::Text(s),
            NodeVal::Boolean(b) => MemNode::Boolean(b),
            NodeVal::Command(cmd) => MemNode::Text(cmd.name()),
            _ => MemNode::Null,
        }
    }
}
