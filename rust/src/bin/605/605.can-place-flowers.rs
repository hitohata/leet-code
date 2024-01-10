/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut bed = flowerbed.clone();
        let mut number_can_be_flowered: i32 = 0;

        if bed.len() == 1 {
            return if bed[0] == 0 {
                return n <= 1;
            } else {
                return n == 0;
            }
        }

        if bed.len() == 2 {
            if bed[0] == 1 || bed[1] == 1 {
                return n == 0;
            } else {
                return n <= 1;
            }
        }

        for i in 0..bed.len() {
            if flowerbed[i] == 1 {
                continue;
            };

            if i == 0 {
                if bed[i] == 0 && bed[i + 1] == 0 {
                    bed[i] = 1;
                    number_can_be_flowered += 1;
                }
                continue;
            }

            if i == bed.len() - 1 {
                if bed[i - 1] == 0 && bed[i] == 0 {
                    number_can_be_flowered += 1;
                }
                continue;
            }

            if bed[i - 1] == 0 && bed[i + 1] == 0 {
                bed[i] = 1;
                number_can_be_flowered += 1;
            };
        };

        number_can_be_flowered >= n
    }
}
// @lc code=end
