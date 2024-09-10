#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}
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

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_sub_path(&head, &root)
    }
}

pub fn is_sub_path(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (head, root) {
        (None, _) => true,
        (_, None) => false,
        (Some(_), Some(tree_node)) => {
            is_same(head, root) || is_sub_path(head, &tree_node.borrow().left) || is_sub_path(head, &tree_node.borrow().right)
        }
    }
}

pub fn is_same(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (head, root) {
        (None, _) => true,
        (_, None) => false,
        (Some(list_node), Some(tree_node)) => {
            list_node.val == tree_node.borrow().val && (is_same(&list_node.next, &tree_node.borrow().left) || is_same(&list_node.next, &tree_node.borrow().right))
        }
    }
}