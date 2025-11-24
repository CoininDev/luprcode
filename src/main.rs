#[macro_use]
mod command;
mod memory;
mod node;
mod node_val;
mod tree;
mod utils;

use command::*;
use node_val::NodeVal;
use tree::Tree;
use utils::print_tree;
use node::NodeRef;

fn main() {
    let t = var_assign();

    println!("Before:");
    print_tree(&t, 0);

    t.borrow_mut().reduce();

    println!("\nAfter:");
    print_tree(&t, 0);
}

// ==========================
// ======== Examples ========
// ==========================

fn simple_comparison() -> NodeRef {
    Tree::root(Eq!()) 
        .l(NodeVal::Num(90))
        .upr(Add!())
            .l(NodeVal::Num(40))
            .upr(NodeVal::Num(50))
    .build()
}

// not working yet
fn var_assign() -> NodeRef {
    Tree::root(NodeVal::Null)    
        .l(Destructor!())
            .l(SetVar!())
                .l(NodeVal::Text("var".into()))
                .upr(NodeVal::Num(256))
                .up()
            .up()
        .upr(GetVar!())
            .l(NodeVal::Text("var".into()))
    .build()
}
