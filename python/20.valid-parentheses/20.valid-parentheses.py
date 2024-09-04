#
# @lc app=leetcode id=20 lang=python3
#
# [20] Valid Parentheses
#

# @lc code=start
class Solution:
    def isValid(self, s: str) -> bool:
        list_s = list(s)
        stack = []

        for s in list_s:
            if s in ["(", "{", "["]:
                stack.insert(0, s)
                continue

            if len(stack) == 0:
                return False
            
            top = stack.pop(0)

            if top == "(" and s == ")":
                continue
            if top == "{" and s == "}":
                continue
            if top == "[" and s == "]":
                continue
            
            return False
        
        return True if len(stack) == 0 else False
# @lc code=end
