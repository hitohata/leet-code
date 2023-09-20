/*
 * @lc app=leetcode id=701 lang=rust
 *
 * [701] Insert into a Binary Search Tree
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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn insert_into_bst(root: Node, val: i32) -> Node {

      match root.clone() {
        None => { return Some(Rc::new(RefCell::new(TreeNode::new(val))))},
        Some(tree) => {
          let mut tree_bor = tree.borrow_mut();
          if tree_bor.val > val {
            tree_bor.left = Self::insert_into_bst(tree_bor.left.clone(), val);
          } else if val > tree_bor.val {
            tree_bor.right = Self::insert_into_bst(tree_bor.right.clone(), val);
          }
          return root.clone()
        }
      }
    }
}
// @lc code=end
