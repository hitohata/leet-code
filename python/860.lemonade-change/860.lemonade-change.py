#
# @lc app=leetcode id=860 lang=python3
#
# [860] Lemonade Change
#

# @lc code=start
class Solution:
    def lemonadeChange(self, bills: List[int]) -> bool:
        map = {
            "5": 0,
            "10": 0,
            "20": 0
        }

        if len(bills) == 0:
            return True
        
        if bills[0] != 5:
            return False

        while len(bills) > 0:
            bill = bills.pop(0)
            if not self.pay_maney(map, bill):
                return False

        return True
        
    
    def pay_maney(self, wallet: Dict[str, int], bill: int) -> bool:
        if bill == 5:
            wallet['5'] = wallet['5'] + 1

        if bill == 10:
            if wallet['5'] == 0:
                return False
            
            wallet['5'] = wallet['5'] - 1
            wallet['10'] = wallet['10'] + 1
    
        if bill == 20:

            if wallet['10'] > 0 and wallet['5'] > 0:
                wallet['20'] = wallet['20'] + 1
                wallet['10'] = wallet['10'] - 1
                wallet['5'] = wallet['5'] - 1
                return True
            
            if wallet['5'] >= 3:
                wallet['5'] = wallet['5'] - 3
                wallet['20'] = wallet['20'] + 1
                return True
            
            return False

        return True
# @lc code=end
