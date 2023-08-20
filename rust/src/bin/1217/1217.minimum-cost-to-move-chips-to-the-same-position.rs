/*
 * @lc app=leetcode id=1217 lang=rust
 *
 * [1217] Minimum Cost to Move Chips to The Same Position
 */


// @lc code=start
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut even = 0;
        let mut odd = 0;
        position.iter().for_each(|el| {
            if el.to_owned() % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        });

        std::cmp::min(even, odd)
    }
}
// @lc code=end
