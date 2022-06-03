// 304. Range Sum Query 2D - Immutable
// https://leetcode.com/problems/range-sum-query-2d-immutable/

class NumMatrix {
    sum: number[][];

    constructor(matrix: number[][]) {
        this.sum = matrix;
        for (let i = 0; i < this.sum.length; i++) {
            for (let j = 1; j < this.sum[0].length; j++) {
                this.sum[i][j] += this.sum[i][j - 1];
            }
        }
        for (let i = 1; i < this.sum.length; i++) {
            for (let j = 0; j < this.sum[0].length; j++) {
                this.sum[i][j] += this.sum[i - 1][j];
            }
        }
    }

    sumRegion(row1: number, col1: number, row2: number, col2: number): number {
        if (row1 === 0) {
            if (col1 === 0) {
                return this.sum[row2][col2];
            } else {
                return this.sum[row2][col2] - this.sum[row2][col1 - 1];
            }
        } else {
            if (col1 === 0) {
                return this.sum[row2][col2] - this.sum[row1 - 1][col2];
            } else {
                return (
                    this.sum[row2][col2] -
                    this.sum[row1 - 1][col2] -
                    this.sum[row2][col1 - 1] +
                    this.sum[row1 - 1][col1 - 1]
                );
            }
        }
    }
}

const m1 = new NumMatrix([
    [3, 0, 1, 4, 2],
    [5, 6, 3, 2, 1],
    [1, 2, 0, 1, 5],
    [4, 1, 0, 1, 7],
    [1, 0, 3, 0, 5],
]);
console.log(m1.sumRegion(2, 1, 4, 3));
console.log(m1.sumRegion(1, 1, 2, 2));
console.log(m1.sumRegion(1, 2, 2, 4));
console.log(m1.sumRegion(0, 2, 2, 4));
console.log(m1.sumRegion(2, 0, 2, 4));

const m2 = new NumMatrix([[-1]]);
console.log(m2.sumRegion(0, 0, 0, 0));
