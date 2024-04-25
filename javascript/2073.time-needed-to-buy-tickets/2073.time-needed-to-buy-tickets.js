/*
 * @lc app=leetcode id=2073 lang=javascript
 *
 * [2073] Time Needed to Buy Tickets
 */

// @lc code=start
/**
 * @param {number[]} tickets
 * @param {number} k
 * @return {number}
 */
var timeRequiredToBuy = function(tickets, k) {
    const a = "a".charCodeAt(0);
    const z = "z".charCodeAt(0);

    const dp = new Array(26).fill(0);

    for (i = 0; i < s.length; i++) {
        let max = 0;
        for (j = Math.max(a, s.charCodeAt(i) - k); j <= Math.min(z, s.charCodeAt(i) + k); j++) {
            max = Math.max(max, dp[j - a]);
        }
        dp[s.charCodeAt(i) - a] = max + 1
    }

    return Math.max(...dp)
};
// @lc code=end
