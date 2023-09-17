/*
 * @lc app=leetcode id=628 lang=rust
 *
 * [628] Maximum Product of Three Numbers
 */

// @lc code=start
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 3 {
            return nums[0] * nums[1] * nums[2];
        };
        let mut sorted = nums.clone();
        sorted.sort();

        std::cmp::max(
            sorted[0] * sorted[1] * sorted[len -1],
            sorted[len - 1] * sorted[len - 2] * sorted[len - 3]
        )
    }
}
// @lc code=end
