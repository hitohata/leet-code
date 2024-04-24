/*
 * @lc app=leetcode id=102 lang=javascript
 *
 * [102] Binary Tree Level Order Traversal
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
var levelOrder = function(root) {
    const queue = [];
    const res = [];

    if (root) {
        queue.push(root)
    };

    while(queue.length > 0) {
        const qLen = queue.length;
        levelVal = [];

        for (i = 0; i < qLen; i++) {
            const node = queue.shift();
            levelVal.push(node.val);
            if (node.left) {
                queue.push(node.left);
            };
            if (node.right) {
                queue.push(node.right);
            }
        }
        res.push(levelVal);
    }

    return res
};
// @lc code=end
