class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        appear = set([])

        i = 0

        for num in nums:
            if num not in appear:
                nums[i] = num
                i += 1
                appear.add(num)
            else:
                continue

        return i
