class Solution:
    def countStudents(self, students: List[int], sandwiches: List[int]) -> int:
        res = len(students)

        map = { 0: 0, 1: 0 }

        for student in students:
            map[student] += 1

        for s in sandwiches:
            if map[s] > 0:
                res -= 1
                map[s] -= 1
            else:
                return res

        return res
