// 1695. Maximum Erasure Value
// https://leetcode.com/problems/maximum-erasure-value/

function maximumUniqueSubarray(nums: number[]): number {
    const map = new Map<number, number>();
    let res = 0;
    let sum = 0;
    let i = 0;
    let j = 0;
    for (; i < nums.length; ) {
        if (!map.has(nums[i])) {
            map.set(nums[i], i);
            sum += nums[i];
        }
        while (j < nums.length - 1) {
            j++;
            if (map.has(nums[j])) {
                break;
            } else {
                map.set(nums[j], j);
                sum += nums[j];
            }
        }
        res = Math.max(res, sum);
        const next = map.get(nums[j]) as number;
        for (let k = i; k <= next; k++) {
            map.delete(nums[k]);
            sum -= nums[k];
        }
        i = next + 1;
        j = Math.max(i, j - 1);
    }
    return res;
}

const bigArray = (m: number, n: number) =>
    [...Array(m)].reduce((a: number[]) => a.concat([...Array(n).keys()].map((i) => i + 1)), []);

const data = [
    [3],
    [4, 2, 4, 5, 6],
    [5, 2, 1, 2, 5, 2, 1, 2, 5],
    [7, 5, 3, 4, 6, 2, 8, 9, 7, 4, 2, 8, 1000],
    bigArray(10, 10000),
];

console.log(data.map(maximumUniqueSubarray));
