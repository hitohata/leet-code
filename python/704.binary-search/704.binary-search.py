#
# @lc app=leetcode id=704 lang=python3
#
# [704] Binary Search
#

# @lc code=start
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        if len(nums) == 1:
            return 0 if nums[0] == target else -1

        l = 0
        r = len(nums) - 1

        while l <= r:
            p = l + ((r - l) // 2)
            center = nums[p]
            if center == target:
                return p

            elif center < target:
                l = p + 1
                continue
            elif center > target:
                r = p - 1

        return -1
# @lc code=end
