/*
 * @lc app=leetcode id=2441 lang=rust
 *
 * [2441] Largest Positive Integer That Exists With Its Negative
 */

use std::cmp::max;

// @lc code=start
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashSet::<i32>::new();
        let mut ans = -1;

        for i in 0..nums.len() {
            if map.contains(&nums[i]) {
                ans = core::cmp::max(ans, nums[i].abs());
            } else {
                map.insert(-1 * nums[i]);
            }
        }

        ans

    }
}
// @lc code=end
