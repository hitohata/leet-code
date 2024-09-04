#
# @lc app=leetcode id=125 lang=python3
#
# [125] Valid Palindrome
#

# @lc code=start
class Solution:
    def isPalindrome(self, s: str) -> bool:
        low = s.lower()

        l = 0
        r = len(low) - 1

        while l <= r:
            
            if not (ord("a") <= ord(low[l]) <= ord("z") or ord("0") <= ord(low[l]) <= ord("9")):
                l += 1
                continue

            if not (ord("a") <= ord(low[r]) <= ord("z") or ord("0") <= ord(low[r]) <= ord("9")):
                r -= 1
                continue

            if low[l] != low[r]:
                return False

            l += 1
            r -= 1
        
        return True
# @lc code=end
