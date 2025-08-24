use crate::node::{Node, NodeID, NodeRef};
use crate::node_val::NodeVal;
use std::rc::Rc;

pub struct Tree {
    root: NodeRef,
    current: NodeRef,
}

impl Tree {
    pub fn root(i: NodeVal) -> Self {
        let root = Node::new(NodeID::Root, i);
        Self {
            root: Rc::clone(&root),
            current: root,
        }
    }

    pub fn l(mut self, i: NodeVal) -> Self {
        let l = Node::new(NodeID::Left, i);
        l.borrow_mut().parent = Some(Rc::downgrade(&self.current));
        self.current.borrow_mut().left = Some(Rc::clone(&l));
        self.current = l;
        self
    }

    pub fn r(mut self, i: NodeVal) -> Self {
        let r = Node::new(NodeID::Right, i);
        r.borrow_mut().parent = Some(Rc::downgrade(&self.current));
        self.current.borrow_mut().right = Some(Rc::clone(&r));
        self.current = r;
        self
    }

    pub fn up(mut self) -> Self {
        let sup = self
            .current
            .borrow()
            .parent
            .as_ref()
            .and_then(|r| r.upgrade())
            .unwrap();
        self.current = sup;
        self
    }

    pub fn upl(mut self, i: NodeVal) -> Self {
        self.up().l(i)
    }
    pub fn upr(mut self, i: NodeVal) -> Self {
        self.up().r(i)
    }

    pub fn build(self) -> NodeRef {
        self.root
    }
}
