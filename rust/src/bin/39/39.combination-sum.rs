/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();

        Self::cal(&candidates, target, &mut answer, &mut current);

        answer
    }
    fn cal(
        candidates: &[i32],
        target: i32,
        answer: &mut Vec<Vec<i32>>,
        current: &mut Vec<i32>
    ) {
        let sum = current.iter().sum::<i32>();

        if sum == target {
            answer.push(current.clone());
            return;
        }

        if sum > target {
            return;
        }

        for i in 0..candidates.len() {
            current.push(candidates[i]);
            Self::cal(&candidates[i..], target, answer, current);
            current.pop();
        }
    }
}
// @lc code=end
