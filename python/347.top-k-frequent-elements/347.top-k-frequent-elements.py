#
# @lc app=leetcode id=347 lang=python3
#
# [347] Top K Frequent Elements
#

# @lc code=start
class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        if k == 0:
            return []

        hash = {}
        count = [[] for i in range(len(nums) + 1)]

        for num in nums:
            hash[num] = hash.get(num, 0) + 1
        for key, v in hash.items():
            count[v].append(key)

        ans = []

        for i in range(len(count) - 1, 0, -1):
            for v in count[i]:
                ans.append(v)
                if len(ans) == k:
                    return ans 

# @lc code=end
