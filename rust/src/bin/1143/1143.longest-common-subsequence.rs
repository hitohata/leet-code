/*
 * @lc app=leetcode id=1143 lang=rust
 *
 * [1143] Longest Common Subsequence
 */

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {

        let char_1 = text1.chars().collect::<Vec<_>>();
        let char_2 = text2.chars().collect::<Vec<_>>();

        let c1_len = char_1.len();
        let c2_len = char_2.len();

        fn count_sub(bigger: Vec<char>, smaller: Vec<char>) -> i32 {
            let bigger_len = bigger.len();
            let small_len = smaller.len();
            let mut count = 0;

            let mut place = 0;

            for aa in 0..small_len {
                let mut c_ = 0;
                for i in 0..bigger_len {
                    let w = bigger[i];
                    for j in place..small_len {
                        if w == smaller[j] {
                            c_ += 1;
                            break;
                        }
                    }
                }
                if c_ > count {
                    count = c_;
                }
                place += 1;
            }

            count
        }

        if c1_len >= c2_len {
            count_sub(char_1, char_2)
        } else {
            count_sub(char_2, char_1)
        }

    }
}
// @lc code=end
