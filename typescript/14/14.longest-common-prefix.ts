/*
 * @lc app=leetcode id=14 lang=typescript
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
function longestCommonPrefix(strs: string[]): string {

    if (strs.length === 0) return "";
    if (strs.length === 1) return strs[0];

    let ans = "";

    for (let i = 0; i < 200; i++) {
        let str = strs[0][i];
        if (typeof str === "undefined") {
            break;
        }
        let is_same = true;

        for (let index = 0; index < strs.length; index++) {
            if (strs[index][i] !== str) {
                is_same = false;
                break;
            }
        }

        if (is_same) {
            ans += str;
        } else {
            break
        }


    }

    return ans;

};
// @lc code=end
