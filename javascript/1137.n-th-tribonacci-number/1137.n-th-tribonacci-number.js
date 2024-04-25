/*
 * @lc app=leetcode id=1137 lang=javascript
 *
 * [1137] N-th Tribonacci Number
 */

const map = new Map();
map.set(0, 0);
map.set(1, 1);
map.set(2, 1);

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var tribonacci = function(n) {
    if (map.has(n)) {
        return map.get(n)
    } else {
        const fib = tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3);
        map.set(n, fib);
        return fib
    }
};
// @lc code=end
