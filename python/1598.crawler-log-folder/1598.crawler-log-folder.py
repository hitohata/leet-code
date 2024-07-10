#
# @lc app=leetcode id=1598 lang=python3
#
# [1598] Crawler Log Folder
#

# @lc code=start
class Solution:
    def minOperations(self, logs: List[str]) -> int:
        depth = 0

        for log in logs:
            if log == "../":
                if depth > 0:
                    depth -= 1
                continue
            if log == "./":
                continue

            depth += 1

        return depth
# @lc code=end
