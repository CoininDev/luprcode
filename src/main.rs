#[macro_use]
mod command;
mod node;
mod node_val;
mod tree;
mod utils;

use command::{Add, Eq};
use node_val::NodeVal;
use tree::Tree;
use utils::print_tree;

fn main() {
    let t = Tree::root(Eq!())
        .l(Add!())
        .l(NodeVal::Num(50))
        .upr(NodeVal::Num(40))
        .up()
        .upr(NodeVal::Num(90))
        .build();

    println!("Antes:");
    print_tree(&t, 0);

    t.borrow_mut().reduce();

    println!("\nDepois:");
    print_tree(&t, 0);
}
