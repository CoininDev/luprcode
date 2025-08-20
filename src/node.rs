use crate::node_val::NodeVal;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeRef = Rc<RefCell<Node>>;
pub type NodeWeak = Weak<RefCell<Node>>;

#[derive(PartialEq, Eq)]
pub enum NodeID {
    Left,
    Right,
    Root,
}

pub struct Node {
    pub id: NodeID,
    pub val: NodeVal,
    pub left: Option<NodeRef>,
    pub right: Option<NodeRef>,
    pub parent: Option<NodeWeak>,
}

impl Node {
    pub fn new(id: NodeID, val: NodeVal) -> NodeRef {
        Rc::new(RefCell::new(Node {
            id,
            val,
            left: None,
            right: None,
            parent: None,
        }))
    }

    pub fn reduce(&mut self) {
        if let Some(l) = &mut self.left {
            l.borrow_mut().reduce();
        }
        if let Some(r) = &mut self.right {
            r.borrow_mut().reduce();
        }

        if let NodeVal::Command(cmd) = &self.val {
            let left_val = self.left.as_ref().map(|l| l.borrow().val.clone());
            let right_val = self.right.as_ref().map(|r| r.borrow().val.clone());
            if let Some(result) = cmd.apply(left_val, right_val) {
                self.val = result;
            } else {
                self.val = NodeVal::Null;
            }
        }
        if let NodeVal::Destructed = &self.val {
            if let Some(a) = &self.parent {
                if let Some(a) = a.upgrade() {
                    let mut a = a.borrow_mut();
                    match self.id {
                        NodeID::Left => a.left = None,
                        NodeID::Right => a.right = None,
                        _ => {}
                    }
                }
            }
        }

        self.left = None;
        self.right = None;
    }
}
