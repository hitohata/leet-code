/*
 * @lc app=leetcode id=198 lang=typescript
 *
 * [198] House Robber
 */

// @lc code=start
function rob(nums: number[]): number {
    if (nums.length === 0) return 0;
    if (nums.length === 1) return nums[0];

    let before = [0, 0];
    let tmp = 0;

    nums.forEach(num => {
        tmp = before[1];
        before[1] = Math.max(before[0] + num, before[1]);
        before[0] = tmp;
    });

    return Math.max(before[0], before[1]);
};
// @lc code=end
