/*
 * @lc app=leetcode id=700 lang=typescript
 *
 * [700] Search in a Binary Search Tree
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function searchBST(root: TreeNode | null, val: number): TreeNode | null {
    return search(root, val);
};

function search(treeNode: TreeNode | null, val: number): TreeNode | null {
    if (!treeNode) {
        return null;
    }

    if (treeNode.val === val) {
        return treeNode;
    }

    if (treeNode.val > val) {
        return search(treeNode.left, val);
    } else  {
        return search(treeNode.right, val);
    }
}
// @lc code=end
