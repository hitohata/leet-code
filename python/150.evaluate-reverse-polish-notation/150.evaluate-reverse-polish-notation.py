#
# @lc app=leetcode id=150 lang=python3
#
# [150] Evaluate Reverse Polish Notation
#

# @lc code=start
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        addition = "+"
        subtraction = "-"
        multiplier = "*"
        dividion = "/"

        stack = []

        for token in tokens:
            
            if token not in [addition, subtraction, multiplier, dividion]:
                stack.append(int(token))
                continue
        
            if token == addition:
                sum = stack.pop() + stack.pop()
                stack.append(sum)

            elif token == subtraction:
                letter = stack.pop()
                reminder = stack.pop() - letter
                stack.append(reminder)

            elif token == multiplier:
                result = stack.pop() * stack.pop()
                stack.append(result)

            else:
                letter = stack.pop()
                result = stack.pop() / letter
                stack.append(math.floor(result) if result > 0 else math.floor(abs(result)) * -1)
                
        return stack.pop()
# @lc code=end
