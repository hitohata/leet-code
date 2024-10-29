#
# @lc app=leetcode id=35 lang=python3
#
# [35] Search Insert Position
#

# @lc code=start
class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        l, r = 0, len(nums) - 1
        m = 0

        while l <= r:
            m = (r + l) // 2
            m_v = nums[m]

            if m_v == target:
                return m
            elif m_v < target:
                l = m + 1
            else:
                r = m - 1

        return l
    # @lc code=end
