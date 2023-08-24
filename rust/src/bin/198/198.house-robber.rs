/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */


// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {

        let nums_len = nums.len();

        if nums_len == 0 { return 0 };
        if nums_len == 1 { return nums[0] };

        let mut before = (0, 0);

        nums.iter().for_each(|el| {
            std::mem::swap(&mut before.0, &mut before.1);
            before.1 = std::cmp::max(before.1 + el, before.0);
        });

        return std::cmp::max(before.0, before.1);

    }
}
// @lc code=end
