use crate::node_val::NodeVal;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeRef = Rc<RefCell<Node>>;
pub type NodeWeak = Weak<RefCell<Node>>;

pub struct Node {
    pub val: NodeVal,
    pub left: Option<NodeRef>,
    pub right: Option<NodeRef>,
    pub parent: Option<NodeWeak>,
}

impl Node {
    pub fn new(val: NodeVal) -> NodeRef {
        Rc::new(RefCell::new(Node {
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
                self.left = None;
                self.right = None;
            }
        }
    }
}
