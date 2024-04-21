/*
 * @lc app=leetcode id=392 lang=javascript
 *
 * [392] Is Subsequence
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isSubsequence = function(s, t) {
    let sPosition = tPosition = 0;


    while (sPosition < s.length && tPosition < t.length) {
        if (s[sPosition] === t[tPosition]) {
            sPosition++
        }
        tPosition++
    }

    return sPosition === s.length
};
// @lc code=end
