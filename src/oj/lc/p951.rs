use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                node1.borrow().val == node2.borrow().val
                    &&
                    (Solution::flip_equiv(node1.borrow().left.clone(), node2.borrow().left.clone()) && Solution::flip_equiv(node1.borrow().right.clone(), node2.borrow().right.clone())
                        ||
                        Solution::flip_equiv(node1.borrow().left.clone(), node2.borrow().right.clone()) && Solution::flip_equiv(node1.borrow().right.clone(), node2.borrow().left.clone()))
            }
            (None, None) => true,
            _ => false
        }
    }
}