/*
 * @lc app=leetcode id=199 lang=javascript
 *
 * [199] Binary Tree Right Side View
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
 * @return {number[]}
 */
var rightSideView = function(root) {
    if (!root) return [];
    const queue = []
    const res = [];

    queue.push(root);

    while (queue.length > 0) {

        const qLen = queue.length;

        val = null

        for (i = 0; i < qLen; i++) {

            const node = queue.shift();

            val = node.val;

            if (node.left) {
                queue.push(node.left);
            }
            if (node.right) {
                queue.push(node.right)
            }

        }

        if (val !== null) {
            res.push(val)
        }

    }

    return res;
};
// @lc code=end
