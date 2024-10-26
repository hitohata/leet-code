#
# @lc app=leetcode id=74 lang=python3
#
# [74] Search a 2D Matrix
#

# @lc code=start
class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        for row in matrix:
            if not(row[0] <= target <= row[-1]):
                continue

            l, r = 0, len(row) - 1
            while l <= r:
                m = (r + l) // 2
                if row[m] == target:
                    return True

                if target < row[m]:
                    r = m - 1
                else:
                    l = m + 1

        return False

# @lc code=end
