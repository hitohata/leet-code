/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();

        if needle_bytes.len() == 0 {
            return -1;
        }

        if haystack_bytes.len() == 0 {
            return -1;
        }

        if haystack_bytes.len() < needle_bytes.len() {
            return -1;
        }

        if haystack_bytes.len() == needle_bytes.len() {
            if haystack_bytes.eq(needle_bytes) {
                return 0
            } else {
                return -1
            }
        }

        if needle_bytes.len() == 1 {
            for i in 0..haystack_bytes.len() {
                if needle_bytes[0].eq(&haystack_bytes[i]) {
                    return i as i32
                }
            }

            return -1
        }

        for i in 0..=(haystack_bytes.len() - needle_bytes.len()) {
            if needle_bytes.eq(&haystack_bytes[i..needle_bytes.len() + i]) {
                return i as i32
            }
        }

        -1
    }
}
// @lc code=end
