/*
 * @lc app=leetcode id=450 lang=rust
 *
 * [450] Delete Node in a BST
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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(t) => {
                let tree = t.borrow_mut();
                if tree.val > key {
                    return Self::delete_node(tree.left.clone(), key);
                } else if tree.val < key {
                    return Self::delete_node(tree.right.clone(), key);
                } else {
                    if tree.left.is_none() && tree.right.is_none() {
                        return None;
                    } else if tree.right.is_none() {
                        return tree.left.clone();
                    } else if tree.left.is_none() {
                        return tree.right.clone();
                    } else {
                    }
                }
                return Some(t);
            }
            Node => {
                return None;
            }
        }
    }

    fn replace(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let tree = root.borrow_mut();
        return if tree.left.is_none() && tree.right.is_none() {
            None
        } else if tree.right.is_none() {
            tree.left.clone()
        } else if tree.left.is_none() {
            tree.right.clone()
        } else {
            let left = tree.left.clone();
            let body = Self::replace(tree);
        };
    }
}
// @lc code=end
