/*
 * @lc app=leetcode id=1768 lang=typescript
 *
 * [1768] Merge Strings Alternately
 */

// @lc code=start
function mergeAlternately(word1: string, word2: string): string {
    let ans = "";
    let len = word1.length > word2.length ? word1.length : word2.length;
    for (let i = 0; i < len; i++) {
        if (word1.length > i) { ans += word1[i] };
        if (word2.length > i) { ans += word2[i] }
    };

    return ans;
};
// @lc code=end
