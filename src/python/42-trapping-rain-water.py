'''42. Trapping Rain Water
https://leetcode.com/problems/trapping-rain-water/
'''

from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        if n < 3:
            return 0
        i = 0
        res = 0
        while i < n - 1:
            init_height = height[i]
            max_height = 0
            heights = 0
            for j in range(i + 1, n):
                curr_height = height[j]
                if curr_height >= init_height:
                    res += (j - i - 1) * init_height - heights
                    i = j
                    if j == n - 1:
                        return res
                    break
                heights += curr_height
                max_height = max(max_height, curr_height)
            if j == n - 1:
                return res + self.trap(list(reversed(height[i:j+1])))

    def test(self):
        for height in [
            [5, 0, 3, 2, 1],
            [5, 0, 3, 1, 1],
            [5, 0, 3, 0, 0],
            [5, 0, 3, 3, 3],
            [5, 4, 3, 3, 3],
            [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            [4, 2, 0, 3, 2, 5],
            [3, 2, 0, 1],
            [5, 0, 3, 0, 1],
        ]:
            print(f'{height} => {self.trap(height)}')


if __name__ == '__main__':
    Solution().test()
