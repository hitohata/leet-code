/*
 * @lc app=leetcode id=643 lang=javascript
 *
 * [643] Maximum Average Subarray I
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var findMaxAverage = function(nums, k) {
    let sum = 0;
    let left = right = 0;

    let max = -Infinity;

    while (right < nums.length) {
        sum += nums[right];
        right++

        if (right < k) continue;

        if (right - k > 0) {
            sum -= nums[left];
            left++
        };
        max = Math.max(max, sum / k);
    }

    return max;
};
// @lc code=end
