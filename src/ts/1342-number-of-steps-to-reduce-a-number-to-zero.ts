// 1342. Number of Steps to Reduce a Number to Zero
// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/

function numberOfSteps(num: number): number {
    if (num <= 3) return num;
    return (num % 2 ? numberOfSteps(num - 1) : numberOfSteps(num / 2)) + 1;
}

console.log([0, 14, 8, 123, 100000].map(numberOfSteps));
