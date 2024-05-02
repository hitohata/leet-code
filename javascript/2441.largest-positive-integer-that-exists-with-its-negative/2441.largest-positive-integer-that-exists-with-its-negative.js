/*
 * @lc app=leetcode id=2441 lang=javascript
 *
 * [2441] Largest Positive Integer That Exists With Its Negative
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var findMaxK = function(nums) {
    let set = new Set();
    let ans = -1;

    for (let i = 0; i < nums.length; i++) {
        if (set.has(nums[i])) {
            if (ans < Math.abs(nums[i])) {
                ans = Math.abs(nums[i])
            }
        } else {
            set.add(-1 * nums[i])
        }
    }

    return ans
};

// @lc code=end
