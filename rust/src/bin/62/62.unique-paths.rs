/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut grid = vec![vec![1; n as usize]; m as usize];


        for i in 1..(m as usize) {
            for j in 1..(n as usize) {
                grid[i][j] = grid[i - 1][j] + grid[i][j - 1];

            };
        }

        grid[(m - 1) as usize][(n - 1) as usize]
    }
}
// @lc code=end
