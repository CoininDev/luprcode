#[macro_use]
mod command;
mod memory;
mod node;
mod node_val;
mod tree;
mod utils;

use command::{Add, Destructor, Eq, GetVar, SetVar};
use node_val::NodeVal;
use tree::Tree;
use utils::print_tree;

fn main() {
    let t = Tree::root(NodeVal::Null)
        .l(Destructor!())
        .l(SetVar!())
        .l(NodeVal::Text("var".into()))
        .upr(NodeVal::Num(256))
        .up()
        .up()
        .upr(GetVar!())
        .l(NodeVal::Text("var".into()))
        .build();

    println!("Antes:");
    print_tree(&t, 0);

    t.borrow_mut().reduce();

    println!("\nDepois:");
    print_tree(&t, 0);
}
