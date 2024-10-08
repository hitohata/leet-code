#
# @lc app=leetcode id=217 lang=python3
#
# [217] Contains Duplicate
#

# @lc code=start
class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        removed_duplicate = set(nums)
        return len(removed_duplicate) != len(nums)
# @lc code=end
