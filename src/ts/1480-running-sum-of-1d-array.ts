// 1480. Running Sum of 1d Array
// https://leetcode.com/problems/running-sum-of-1d-array/

function runningSum(nums: number[]): number[] {
    return nums.reduce((x: number[], i) => {
        x.push((x[x.length - 1] || 0) + i);
        return x;
    }, []);
}

console.log(
    [
        [1, 2, 3, 4],
        [1, 1, 1, 1, 1],
        [3, 1, 2, 10, 1],
    ].map(runningSum)
);
