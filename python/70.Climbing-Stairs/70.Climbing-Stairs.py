class Solution:
    def climbStairs(self, n: int) -> int:
        if n <= 3:
            return n

        mem = [0] * (n + 1)

        for i in range(n + 1):
            if i <= 1:
                mem[i] = 1
            else:
                mem[i] = mem[i - 1] + mem[i - 2]

        return mem[-1]
