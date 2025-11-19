"""2094. Finding 3-Digit Even Numbers
https://leetcode.com/problems/finding-3-digit-even-numbers/description/
"""

from itertools import permutations
from typing import List


class Solution:
    @staticmethod
    def filter_func(x: tuple[int, int, int]):
        return x[0] != 0 and x[2] % 2 == 0

    @staticmethod
    def map_func(x: tuple[int, int, int]):
        return x[0] * 100 + x[1] * 10 + x[2]

    def findEvenNumbers(self, digits: List[int]) -> List[int]:
        return sorted(
            set(map(self.map_func, filter(self.filter_func, permutations(digits, 3))))
        )


for digits in [
    [2, 1, 3, 0],
    [2, 2, 8, 8, 2],
    [3, 7, 5],
]:
    print(Solution().findEvenNumbers(digits))
