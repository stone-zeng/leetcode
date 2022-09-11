// 1383. Maximum Performance of a Team
// https://leetcode.com/problems/maximum-performance-of-a-team/

function maxPerformance(n: number, speed: number[], efficiency: number[], k: number): number {
    const modulo = 10 ** 9 + 7;

    const pairs = speed.map((s, i) => [s, efficiency[i]]).sort((p1, p2) => p1[1] - p2[1]);
    let res = 0;

    // pairs.forEach(([s, e], i) => {
    //     const sum = sumLargest(
    //         pairs.slice(i + 1).map((p) => p[0]),
    //         Math.min(k - 1, n - i)
    //     );
    //     res = Math.max(res, (sum + s) * e);
    // });

    const speedSorted = speed.sort((a, b) => b - a);
    pairs.forEach(([s, e], i) => {
        speedSorted.splice(speedSorted.indexOf(s), 1);
        const sum = speedSorted.slice(0, Math.min(k - 1, n - i)).reduce((i, s) => i + s, 0);
        res = Math.max(res, (sum + s) * e);
    });

    return Number(res % modulo);
}

// const sumLargest = (nums: number[], n: number) => {
//     if (!n) return 0;
//     return nums
//         .sort((a, b) => b - a)
//         .slice(0, n)
//         .reduce((i, s) => i + s, 0);
// };

console.log(
    [
        { n: 6, speed: [2, 10, 3, 1, 5, 8], efficiency: [5, 4, 3, 9, 7, 2], k: 2 },
        { n: 6, speed: [2, 10, 3, 1, 5, 8], efficiency: [5, 4, 3, 9, 7, 2], k: 3 },
        { n: 6, speed: [2, 10, 3, 1, 5, 8], efficiency: [5, 4, 3, 9, 7, 2], k: 4 },
    ].map(({ n, speed, efficiency, k }) => maxPerformance(n, speed, efficiency, k))
);
