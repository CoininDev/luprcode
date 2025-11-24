#![allow(dead_code)]
#![allow(unused_macros)]

use crate::{
    memory::{MEMORY, MemNode},
    node_val::NodeVal,
};

pub trait CommandStrategy {
    fn apply(&self, left: Option<NodeVal>, right: Option<NodeVal>) -> Option<NodeVal>;
    fn name(&self) -> String;
}

// Helpers
fn to_bool(v: &NodeVal) -> bool {
    match v {
        NodeVal::Boolean(b) => *b,
        NodeVal::Num(n) => *n != 0,
        _ => false,
    }
}

// Estratégias concretas
macro_rules! strategy_struct {
    ($name:ident, $apply:expr) => {
        pub struct $name;
        impl crate::command::CommandStrategy for $name {
            fn apply(&self, left: Option<NodeVal>, right: Option<NodeVal>) -> Option<NodeVal> {
                $apply(left, right)
            }
            fn name(&self) -> String {
                stringify!($name).to_string()
            }
        }
        macro_rules! $name {
            () => {
                NodeVal::Command(Box::new($name {}))
            };
        }
    };
}

//mathematical operations
strategy_struct!(Add, |l, r| match (l?, r?) {
    (NodeVal::Num(a), NodeVal::Num(b)) => Some(NodeVal::Num(a + b)),
    (NodeVal::Text(a), NodeVal::Text(b)) => Some(NodeVal::Text(format!("{}{}", a, b))),

    (NodeVal::Null, _) | (_, NodeVal::Null) => Some(NodeVal::Null),
    _ => None,
});

strategy_struct!(Sub, |l, r| match (l?, r?) {
    (NodeVal::Num(a), NodeVal::Num(b)) => Some(NodeVal::Num(a - b)),

    (NodeVal::Null, _) | (_, NodeVal::Null) => Some(NodeVal::Null),
    _ => None,
});

strategy_struct!(Mul, |l, r| match (l?, r?) {
    (NodeVal::Num(a), NodeVal::Num(b)) => Some(NodeVal::Num(a * b)),

    (NodeVal::Null, _) | (_, NodeVal::Null) => Some(NodeVal::Null),
    _ => None,
});

strategy_struct!(Div, |l, r| match (l?, r?) {
    (NodeVal::Num(a), NodeVal::Num(b)) if b != 0 => Some(NodeVal::Num(a / b)),

    (NodeVal::Null, _) | (_, NodeVal::Null) => Some(NodeVal::Null),
    _ => None,
});

strategy_struct!(Eq, |l, r| Some(NodeVal::Boolean(l? == r?)));

//boolean
strategy_struct!(Not, |l, _| match l? {
    NodeVal::Boolean(b) => Some(NodeVal::Boolean(!b)),
    NodeVal::Num(n) => Some(NodeVal::Boolean(n == 0)),
    _ => None,
});

strategy_struct!(And, |l, r| Some(NodeVal::Boolean(
    to_bool(&l?) && to_bool(&r?)
)));
strategy_struct!(Or, |l, r| Some(NodeVal::Boolean(
    to_bool(&l?) || to_bool(&r?)
)));
strategy_struct!(Xor, |l, r| Some(NodeVal::Boolean(
    to_bool(&l?) ^ to_bool(&r?)
)));
strategy_struct!(Xand, |l, r| Some(NodeVal::Boolean(
    !(to_bool(&l?) && to_bool(&r?))
)));
strategy_struct!(IfCmd, |l, r: Option<NodeVal>| match l? {
    NodeVal::Boolean(true) => r.clone(),
    NodeVal::Boolean(false) => Some(NodeVal::Destructed),
    _ => Some(NodeVal::Destructed),
});

//vars
strategy_struct!(SetVar, |l: Option<NodeVal>, r: Option<NodeVal>| {
    let mut mem = MEMORY.lock().ok()?;
    let key: MemNode = l?.into();
    let value: MemNode = r.clone()?.into();
    mem.map.insert(key, value);
    Some(r?)
});

strategy_struct!(GetVar, |l: Option<NodeVal>, _| {
    let mem = MEMORY.lock().ok()?;
    let key: MemNode = l?.into();
    let res: NodeVal = mem.map[&key].clone().into();
    Some(res)
});

strategy_struct!(Destructor, |_, _| { Some(NodeVal::Destructed) });
