// 867. Transpose Matrix
// https://leetcode.com/problems/transpose-matrix/

function transpose(matrix: number[][]): number[][] {
    const res = [];
    for (let j = 0; j < matrix[0].length; j++) {
        const row = [];
        for (let i = 0; i < matrix.length; i++) {
            row.push(matrix[i][j]);
        }
        res.push(row);
    }
    return res;
}

console.log(
    transpose([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ])
);
console.log(
    transpose([
        [1, 2, 3],
        [4, 5, 6],
    ])
);
