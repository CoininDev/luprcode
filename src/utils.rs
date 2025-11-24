use crate::node::NodeRef;

pub fn print_tree(node: &NodeRef, depth: usize) {
    for _ in 0..depth { print!("  "); }
    println!("{}", node.borrow().val);
    if let Some(ref left) = node.borrow().left { print_tree(left, depth + 1); }
    if let Some(ref right) = node.borrow().right { print_tree(right, depth + 1); }
}
