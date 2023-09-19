/*
 * @lc app=leetcode id=136 lang=typescript
 *
 * [136] Single Number
 */

// @lc code=start
function singleNumber(nums: number[]): number {
    return nums.reduce((acc, x) => acc ^ x, 0);
};
// @lc code=end
