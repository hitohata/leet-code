/*
 * @lc app=leetcode id=62 lang=typescript
 *
 * [62] Unique Paths
 */

// @lc code=start
function uniquePaths(m: number, n: number): number {
    const grid = new Array(n).fill(1);

    for (let i = 1; i < m; i++) {
        for (let j = 1; j < n; j++) {
            grid[j] = grid[j] + grid[j - 1];
        }
    }

    return grid[n - 1]

};
// @lc code=end
