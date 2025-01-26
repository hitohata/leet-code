class Solution:
    def isHappy(self, n: int) -> bool:
        
        if n == 1:
            return True

        m = {}

        while not n in m:
            m[n] = 1
            n_str = str(n)
            sum = 0

            for i in n_str:
                sum += int(i) ** 2

            if sum == 1:
                return True

            n = sum

        return False
