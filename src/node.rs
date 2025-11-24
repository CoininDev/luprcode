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
        let mut remove_children: bool = false;

        if let Some(l) = &mut self.left {
            { l.borrow_mut().reduce(); }

            if matches!(l.borrow().val, NodeVal::Destructed) {
                self.left = None;
            }
        }
        if let Some(r) = &mut self.right {
            { r.borrow_mut().reduce(); }

            if matches!(r.borrow().val, NodeVal::Destructed) {
                self.right = None;
            }
        }

        if let NodeVal::Command(cmd) = &self.val {
            let left_val = self.left.as_ref().map(|l| l.borrow().val.clone());
            let right_val = self.right.as_ref().map(|r| r.borrow().val.clone());
            if let Some(result) = cmd.apply(left_val, right_val) {
                self.val = result;
            } else {
                self.val = NodeVal::Null;
            }
            remove_children = true;
        }

        if remove_children{
            self.right = None;
            self.left = None;
        }
    }
}
