/*
 * @lc app=leetcode id=2405 lang=javascript
 *
 * [2405] Optimal Partition of String
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var partitionString = function(s) {
    let set = new Set();
    let p = 1;

    for (i = 0; i < s.length; i++) {
        if (set.has(s[i])) {
            set.clear();
            p++;
        }
        set.add(s[i]);
    }
    return p
};
// @lc code=end
