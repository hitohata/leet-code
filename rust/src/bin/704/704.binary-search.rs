/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        let mut left = 0_usize;
        let mut right = nums.len() - 1;

        loop {

            if left == right {
                return -1
            }

            let mid = (right - left) / 2;

            if nums[mid] == target {
                return mid as i32
            }

            if nums[mid] > target {
                left = mid;
            } else {
                right = mid;
            }
        }
    }
}
// @lc code=end
