// 985. Sum of Even Numbers After Queries
// https://leetcode.com/problems/sum-of-even-numbers-after-queries/

function sumEvenAfterQueries(nums: number[], queries: number[][]): number[] {
    let sum = nums.reduce((s, i) => (i % 2 ? 0 : i) + s, 0);
    const res: number[] = [];
    queries.forEach(([val, index]) => {
        sum += nums[index] % 2 ? (val % 2 ? nums[index] + val : 0) : val % 2 ? -nums[index] : val;
        nums[index] += val;
        res.push(sum);
    });
    return res;
}

[
    {
        nums: [1, 2, 3, 4],
        queries: [
            [1, 0],
            [-3, 1],
            [-4, 0],
            [2, 3],
        ],
    },
    { nums: [1], queries: [[4, 0]] },
].forEach(({ nums, queries }) => console.log(sumEvenAfterQueries(nums, queries)));
