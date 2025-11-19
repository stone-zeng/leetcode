// 1383. Maximum Performance of a Team
// https://leetcode.com/problems/maximum-performance-of-a-team/

function maxPerformance(n: number, speed: number[], efficiency: number[], k: number): number {
    const modulo = BigInt(10 ** 9 + 7);
    const pairs = speed.map((s, i) => [s, efficiency[i]]).sort((p1, p2) => p1[1] - p2[1]);
    const speedSorted = speed.sort((a, b) => b - a);
    let sum = speedSorted.slice(0, k - 1).reduce((i, s) => i + s, 0);
    let res = 0n;
    pairs.forEach(([s, e]) => {
        const pos = binarySearch(speedSorted, s);
        speedSorted.splice(pos, 1);
        if (pos < k - 1) {
            sum = sum - s + (k - 1 > speedSorted.length ? 0 : speedSorted[k - 2]);
        }
        res = max(res, BigInt(sum + s) * BigInt(e));
    });
    return Number(res % modulo);
}

const binarySearch = (nums: number[], n: number) => {
    let begin = 0;
    let end = nums.length;
    while (begin + 1 <= end) {
        const mid = Math.floor((begin + end) / 2);
        const numMid = nums[mid];
        if (numMid === n) return mid;
        if (numMid < n) {
            end = mid;
        } else {
            begin = mid + 1;
        }
    }
    return -1;
};

const max = <T>(a: T, b: T) => (a > b ? a : b);

import data from '../data.json';

console.log(
    [
        { n: 6, speed: [2, 10, 3, 1, 5, 8], efficiency: [5, 4, 3, 9, 7, 2], k: 2 },
        { n: 6, speed: [2, 10, 3, 1, 5, 8], efficiency: [5, 4, 3, 9, 7, 2], k: 3 },
        { n: 6, speed: [2, 10, 3, 1, 5, 8], efficiency: [5, 4, 3, 9, 7, 2], k: 4 },
        data,
    ].map(({ n, speed, efficiency, k }) => maxPerformance(n, speed, efficiency, k))
);
