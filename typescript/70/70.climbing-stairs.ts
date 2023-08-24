/*
 * @lc app=leetcode id=70 lang=typescript
 *
 * [70] Climbing Stairs
 */

// @lc code=start
function climbStairs(n: number): number {
    if (n == 1) {
        return 1
    }

    if (n == 2) {
        return 2
    }

    let step = [0, 1];
    let tmp = 0;

    for (let i = 0; i < n; i++) {
        tmp = step[1];
        step[1] += step[0];
        step[0] = tmp;
    }

    return step[1];
};
// @lc code=end
