#
# @lc app=leetcode id=121 lang=python3
#
# [121] Best Time to Buy and Sell Stock
#

# @lc code=start
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        min = prices[0]

        for price in prices:
            if price < min:
                min = price

            if profit < price - min:
                profit = price - min


        return profit

# @lc code=end
