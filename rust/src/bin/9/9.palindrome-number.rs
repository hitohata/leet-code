/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string().as_bytes().to_owned();
        let x_len = x_str.len();

        if x_len <= 1 {
            return true;
        };

        let center = x_len / 2;

        if x_len % 2 == 0 {
            check(x_str, x_len, center)
        } else {
            check(x_str, x_len, center)
        }

    }
}

fn check(origin: Vec<u8>, len: usize, limit: usize) -> bool {

    for i in 0..limit {
        if origin[i] != origin[len - 1 - i] {
            return false;
        }
    }

    true
}
// @lc code=end
