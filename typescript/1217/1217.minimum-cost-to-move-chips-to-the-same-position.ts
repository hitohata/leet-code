/*
 * @lc app=leetcode id=1217 lang=typescript
 *
 * [1217] Minimum Cost to Move Chips to The Same Position
 */

// @lc code=start
function minCostToMoveChips(position: number[]): number {

    let odd = 0;
    let even = 0;

    position.forEach(el => {
        el % 2 == 0 ? even += 1 : odd += 1
    });

    return Math.min(odd, even);

};
// @lc code=end
