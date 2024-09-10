#
# @lc app=leetcode id=739 lang=python3
#
# [739] Daily Temperatures
#

# @lc code=start
class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        temp = []
        ans = [0 for i in range(len(temperatures))]

        for i, t in enumerate(temperatures):
            if len(temp) == 0:
                temp.append((t, i))
                continue

            for ii in range(len(temp), 0, -1):
                if temp[ii - 1][0] < t:
                    ans[temp[ii - 1][1]] = i - temp[ii - 1][1]
                    temp.pop()
                else:
                    break

            temp.append((t, i))


        return ans
# @lc code=end
