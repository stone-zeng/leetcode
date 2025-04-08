"""3396. Minimum Number of Operations to Make Elements in Array Distinct
https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/
"""

from typing import List


class Solution:
    def minimumOperations(self, nums: List[int]) -> int:
        if len(set(nums)) == len(nums):
            return 0
        if len(nums) <= 3:
            return 1
        return 1 + self.minimumOperations(nums[3:])


if __name__ == "__main__":
    for nums in [
        [1, 2, 3, 4, 2, 3, 3, 5, 7],
        [4, 5, 6, 4, 4],
        [6, 7, 8, 9],
    ]:
        print(Solution().minimumOperations(nums))
