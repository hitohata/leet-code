/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */


// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {

        let mut swap_point = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, swap_point);
                swap_point += 1
            }
        }
    }
}
// @lc code=end
