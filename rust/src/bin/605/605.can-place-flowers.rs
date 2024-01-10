/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut bed = flowerbed.clone();

        bed.push(0);
        bed.insert(0, 0);

        let total = bed.split(|&f| f == 1).map(|f| (f.len() - 1) / 2).sum::<usize>();

        total >= (n as usize)
    }
}
// @lc code=end
