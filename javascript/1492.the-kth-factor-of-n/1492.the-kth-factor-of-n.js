/*
 * @lc app=leetcode id=1492 lang=javascript
 *
 * [1492] The kth Factor of n
 */

// @lc code=start
/**
 * @param {number} n
 * @param {number} k
 * @return {number}
 */
var kthFactor = function(n, k) {
    const factor = [];
    for (i = 1; i <= n; i++) {
        if (n % i === 0) {
            factor.push(i)
        }
    }
    if (factor.length >= k) {
        return factor[k - 1]
    } else {
        return -1
    }
};
// @lc code=end
