/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let mut strs_vec = vec![];
        let mut min_len = std::usize::MAX;

        strs.iter().for_each(|el| {
            let a = el.chars();
            let vec_char = el.chars().collect::<Vec<char>>();
            strs_vec.push(vec_char.clone());
            if vec_char.len() < min_len {
                min_len = vec_char.len();
            }
        });

        let mut ans = vec![];

        for i in 0..min_len {
            let p = strs_vec[0][i];
            let mut is_same = true;

            for j in 0..strs_vec.len() {
                if p != strs_vec[j][i] {
                    is_same = false;
                    break;
                }
            }
            if is_same {
                ans.push(p);
            } else {
                break;
            }
        }

        ans.iter().collect::<String>()

    }
}
// @lc code=end
