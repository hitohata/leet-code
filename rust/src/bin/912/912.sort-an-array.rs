/*
 * @lc app=leetcode id=912 lang=rust
 *
 * [912] Sort an Array
 */

// @lc code=start
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {

        let mut answer = nums.clone();

        for i in 1..nums.len() {

            let mut j: usize = i - 1;

            loop {
                if answer[j + 1] < answer[j] {
                    answer.swap(j + 1, j);
                } else {
                    break;
                };

                if j == 0 {break};

                j -= 1;
            }

        }

        answer
    }
}
// @lc code=end
