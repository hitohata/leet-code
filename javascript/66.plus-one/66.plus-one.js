/*
 * @lc app=leetcode id=66 lang=javascript
 *
 * [66] Plus One
 */

// @lc code=start
/**
 * @param {number[]} digits
 * @return {number[]}
 */
var plusOne = function(digits) {
    let dig = digits;
    let ex = false;

    for (let i = digits.length - 1; i >= 0; i--) {
        let sum = dig[i] + 1;
        ex = !!(Math.floor(sum / 10));
        dig[i] = Math.floor(sum % 10);
        if (!ex) break;
    };

    if (ex) {
        dig.unshift(1);
    }

    return dig
};
// @lc code=end
