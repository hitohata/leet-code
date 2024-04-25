/*
 * @lc app=leetcode id=704 lang=javascript
 *
 * [704] Binary Search
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function(nums, target) {
    let left = 0;
    let right = nums.length - 1;
    let res = -1;

    while (right - left >= 0) {
        let m = Math.trunc((right + left) / 2);

        if (nums[m] === target) {
            res = m;
            break;
        }

        if (right === left) break;

        if (nums[m] < target) {
            left = m + 1;
        } else {
            right = m;
        }
    }

    return res
};
// @lc code=end
