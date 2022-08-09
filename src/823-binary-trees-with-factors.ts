// 823. Binary Trees With Factors
// https://leetcode.com/problems/binary-trees-with-factors/

function numFactoredBinaryTrees(arr: number[]): number {
    const modulo = 10 ** 9 + 7;
    const factors = new Map<number, number>();
    arr.sort((a, b) => a - b).forEach((elem) => {
        let count = 1;
        factors.forEach((value, key) => {
            count += (factors.get(elem / key) || 0) * value;
        });
        factors.set(elem, count);
    });
    return [...factors.values()].reduce((s, i) => s + (i % modulo), 0) % modulo;
}

console.log(
    [
        [2, 4],
        [5, 4, 2, 10],
        [48, 72, 144, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 36],
    ].map(numFactoredBinaryTrees)
);
