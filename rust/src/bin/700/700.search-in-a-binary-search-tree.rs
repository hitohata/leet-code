/*
 * @lc app=leetcode id=700 lang=rust
 *
 * [700] Search in a Binary Search Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::search(root, val)
    }

    fn search(tree: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match tree {
            None => None,
            Some(treeNode) => {
                let current_val = treeNode.borrow().val;
                if current_val > val {
                    Self::search(treeNode.borrow().left.clone(), val)
                } else if current_val < val {
                    Self::search(treeNode.borrow().right.clone(), val)
                } else{
                    Some(treeNode)
                }
            }
        }
    }
}
// @lc code=end
