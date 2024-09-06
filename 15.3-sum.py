#
# @lc app=leetcode id=15 lang=python3
#
# [15] 3Sum
#

# @lc code=start
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        ans = []

        nums.sort()

        for i, n in enumerate(nums):
            
            if n > 0:
                break
            
            if i > 0 and n == nums[i - 1]:
                continue
            
            l = i + 1
            r = len(nums) - 1

            while (l < r):
                sum = n + nums[l] + nums[r]

                if sum > 0:
                    r -= 1
                elif sum < 0:
                    l += 1
                else:
                    ans.append([n, nums[l], nums[r]])
                    r -= 1
                    l += 1
                    while (nums[l] == nums[l - 1]) and l < r:
                        l += 1


        return ans
# @lc code=end
