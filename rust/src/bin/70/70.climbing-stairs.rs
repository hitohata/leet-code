/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut arr = vec![0; n as usize + 2];
        arr[1] = 1;
        arr[2] = 2;

        if n <= 2 {
            return n
        }

        for i in 3..=(n as usize)+1 {
            arr[i] = arr[i - 1] + arr[i - 2]; 
        }

        return arr[n as usize]
    }
}
// @lc code=end
