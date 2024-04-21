/*
 * @lc app=leetcode id=167 lang=javascript
 *
 * [167] Two Sum II - Input Array Is Sorted
 */

// @lc code=start
/**
 * @param {number[]} numbers
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(numbers, target) {
    const map = new Map();

    for (let i = 0; i < numbers.length; i++) {
        const substructionResult = target - numbers[i];
        const mapVal = map.get(substructionResult);
        if (mapVal >= 0) {
            return [mapVal + 1, i + 1];
        }
        map.set(numbers[i], i)
    }
    return [0, 0]
};
// @lc code=end
