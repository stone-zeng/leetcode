// 47. Permutations II
// https://leetcode.com/problems/permutations-ii/

function permuteUnique(nums: number[]): number[][] {
    const res = [nums.sort((a, b) => a - b)];
    while (true) {
        const next = nextPermutation(res.slice(-1)[0]);
        if (!next) return res;
        res.push(next);
    }
}

function nextPermutation(nums: number[]): number[] | null {
    let k = -1;
    for (let i = nums.length - 1; i >= 0; i--) {
        if (nums[i] < nums[i + 1]) {
            k = i;
            break;
        }
    }
    if (k === -1) return null;

    let l = -1;
    for (let i = nums.length - 1; i > k; i--) {
        if (nums[k] < nums[i]) {
            l = i;
            break;
        }
    }

    const next = [...nums];
    const temp = next[k];
    next[k] = next[l];
    next[l] = temp;

    return next.slice(0, k + 1).concat(next.slice(k + 1).reverse());
}

[[1], [1, 2], [1, 1, 2], [1, 2, 3], [2, 1, 1, 3]].forEach((nums) =>
    console.log(permuteUnique(nums))
);
