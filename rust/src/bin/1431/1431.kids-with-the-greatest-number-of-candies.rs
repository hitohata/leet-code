/*
 * @lc app=leetcode id=1431 lang=rust
 *
 * [1431] Kids With the Greatest Number of Candies
 */

// @lc code=start
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result = vec![false; candies.len()];

        let max = candies.iter().max().unwrap().to_owned();

        for (index, candy) in candies.iter().enumerate() {
            if candy + extra_candies >= max {
                result[index] = true
            }
        }

        result
    }
}
// @lc code=end

