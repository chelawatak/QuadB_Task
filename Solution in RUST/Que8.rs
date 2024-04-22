// Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn with_children(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val,
            left,
            right,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            let left_depth = max_depth(node_ref.left.clone());
            let right_depth = max_depth(node_ref.right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example usage:
    let root = Some(Rc::new(RefCell::new(TreeNode::with_children(
        3,
        Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        Some(Rc::new(RefCell::new(TreeNode::with_children(
            20,
            Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        )))),
    ))));

    let depth = max_depth(root);
    println!("Maximum depth of the tree: {}", depth);
}
