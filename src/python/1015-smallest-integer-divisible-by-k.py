"""1015. Smallest Integer Divisible by K
https://leetcode.com/problems/smallest-integer-divisible-by-k/
"""


class Solution:
    def smallestRepunitDivByK(self, k: int) -> int:
        if k == 1:
            return 1
        if k % 2 == 0 or k % 5 == 0:
            return -1

        n = 1
        num = 1
        while num % k != 0:
            num = (10 * num + 1) % k
            n += 1
        return n


if __name__ == "__main__":
    sol = Solution()
    for i in [1, 2, 3, 32767, 19927, 65537]:
        print(sol.smallestRepunitDivByK(i))
