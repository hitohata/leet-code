/*
 * @lc app=leetcode id=1343 lang=javascript
 *
 * [1343] Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold
 */

// @lc code=start
/**
 * @param {number[]} arr
 * @param {number} k
 * @param {number} threshold
 * @return {number}
 */
var numOfSubarrays = function(arr, k, threshold) {
    if (arr.langth < k) return 0;

    let result = 0;
    let sum = 0;

    for (let i = 0; i < arr.length; i++) {

        sum += arr[i];

        if (i + 1 < k) continue;

        if (i - k >= 0) {
            sum -= arr[i - k];
        };

        if (sum / k >= threshold) {
            result += 1;
        }
    }

    return result
};
// @lc code=end
