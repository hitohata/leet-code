/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.to_string().chars().rev().eq(x.to_string().chars()) {
            true
        } else {
            false
        }
    }
}
// @lc code=end
