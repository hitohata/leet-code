#
# @lc app=leetcode id=36 lang=python3
#
# [36] Valid Sudoku
#

# @lc code=start
class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        row = defaultdict(set)
        col = defaultdict(set)
        sq = defaultdict(set)

        for r in range(0, 9):
            for c in range(0, 9):
                val = board[r][c]

                if val == ".":
                    continue
                
                if val in row[r] or val in col[c] or val in sq[(r // 3, c // 3)]:
                    return False

                row[r].add(val)
                col[c].add(val)
                sq[(r // 3, c // 3)].add(val)

# @lc code=end
