/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut step = [1, 2];

        if n < 2 {
            return 1
        }

        for i in 0..n {

            if i < 2 {
                continue;
            }

            let tmp = step[1];
            step[1] += step[0];
            step[0] = tmp;

        }

        step[1]
    }
}
// @lc code=end
