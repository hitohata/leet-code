#
# @lc app=leetcode id=49 lang=python3
#
# [49] Group Anagrams
#

# @lc code=start
class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        if len(strs) < 1:
            return [strs]

        ans = defaultdict(list)
        
        for s in strs:
            count = [0] * 26

            for c in s:
                i = ord(c) - ord('a')
                count[i] += 1

            ans[tuple(count)].append(s)

        return ans.values()

# @lc code=end
