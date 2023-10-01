/* * @lc app=leetcode id=283 lang=typescript *
 * [283] Move Zeroes
 */

// @lc code=start
/**
 Do not return anything, modify nums in-place instead.
 */
function moveZeroes(nums: number[]): void {

    let write = 0;

    for (let i = 0; i < nums.length; i++) {

        if (nums[i] != 0) {
            const temp: number = nums[write];
            nums[write] = nums[i];
            nums[i] = temp;
            write++;
        }

    }

};
// @lc code=end
