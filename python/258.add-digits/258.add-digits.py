#
# @lc app=leetcode id=258 lang=python3
#
# [258] Add Digits
#

# @lc code=start
class Solution:
    def addDigits(self, num: int) -> int:
        num_str = str(num)

        while len(num_str) > 1:
            sum = 0
            for i in num_str:
                sum += int(i)

            num_str = str(sum)

        return int(num_str)

# @lc code=end
