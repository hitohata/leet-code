/*
 * @lc app=leetcode id=345 lang=javascript
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string}
 */
var reverseVowels = function(s) {
    let left = 0;
    let right = s.length - 1;

    let sArr = s.split('')

    while (right - left > 0) {
        if (!(isVowel(sArr[right].charCodeAt()))) {
            right--
        } else if (!(isVowel(sArr[left].charCodeAt()))) {
            left++
        } else {
            const tmp = sArr[left];
            sArr[left] = sArr[right];
            sArr[right] = tmp;
            right--;
            left++;
        }
    }

    return sArr.join('')
};
// @lc code=end

const isVowel = (num) => {
    return !!(num === 65 || num === 69 || num === 73 || num === 79 || num === 85 || num === 97 || num === 101 || num === 105 || num === 111 || num === 117)
}
