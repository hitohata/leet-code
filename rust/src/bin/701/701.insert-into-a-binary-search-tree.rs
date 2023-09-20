/*
 * @lc app=leetcode id=701 lang=rust
 *
 * [701] Insert into a Binary Search Tree
 */

// @lc code=start
// Definition for a binary tree node.
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
      right: None
    }
  }
}
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
      Self::insertion(root, val)
    }

    fn insertion(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
      return match root {
        None => { Some(Rc::new(RefCell::new(TreeNode::new(val))))},
        Some(tree) => {
          if tree.borrow().val > val {
            tree.borrow_mut().left = Self::insertion(tree.borrow().left.clone(), val);
          } else if val > tree.borrow().val {
            tree.borrow_mut().right = Self::insertion(tree.borrow().right.clone(), val);
          }
          root
        }
      }
    }
}
// @lc code=end
