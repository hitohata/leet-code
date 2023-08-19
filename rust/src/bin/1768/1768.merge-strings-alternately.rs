/*
 * @lc app=leetcode id=1768 lang=rust
 *
 * [1768] Merge Strings Alternately
 */

// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut answer = Vec::<char>::new();

        let word1_char = word1.chars().collect::<Vec<char>>();
        let word2_char = word2.chars().collect::<Vec<char>>();

        for i in 0..(if word1_char.len() > word2_char.len() {word1_char.len()} else {word2_char.len()}) {

            if word1_char.len() > i {
                answer.push(word1_char[i].to_owned());
            }
            if word2_char.len() > i {
                answer.push(word2_char[i].to_owned());
            }

        }

        answer.iter().collect::<String>()
    }
}
// @lc code=end
