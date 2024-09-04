#
# @lc app=leetcode id=238 lang=python3
#
# [238] Product of Array Except Self
#

# @lc code=start
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        zero_ind = -1
        product = 1

        for ind, el in enumerate(nums):
            if el == 0:
                if zero_ind != -1:
                    return [0 for i in range(len(nums))]
                zero_ind = ind
            else:
                product *= el

        if zero_ind != -1:
            ans = []

            for el in range(len(nums)):
                if el == zero_ind:
                    ans.append(product)
                else:
                    ans.append(0)

            return ans

        else:
            return list(map(lambda x: product // x, nums))
# @lc code=end
