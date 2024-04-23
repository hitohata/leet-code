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
    const sList = s.split("");

    let map = new Map();
    let p = 1;

    for (i = 0; i < sList.length; i++) {
        const res = map.get(sList[i]);
        if (res) {
            map = new Map();
            p++;
        }
        map.set(sList[i], true); 
    }
    return p
};
// @lc code=end
