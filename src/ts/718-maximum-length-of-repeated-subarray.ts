// 718. Maximum Length of Repeated Subarray
// https://leetcode.com/problems/maximum-length-of-repeated-subarray/

function findLength(nums1: number[], nums2: number[]): number {
    let max_len = 0;
    for (let i = 0; i < nums1.length - max_len; i++) {
        for (let j = -1; j < nums2.length; ) {
            const next = nums2.slice(j + 1).findIndex((e) => e === nums1[i]);
            j = next >= 0 ? j + next + 1 : -1;
            if (j >= 0) {
                let temp = 1;
                for (let n = 1; n < Math.min(nums1.length - i, nums2.length - j); n++) {
                    if (nums1[i + n] === nums2[j + n]) {
                        temp += 1;
                    } else {
                        break;
                    }
                }
                max_len = Math.max(max_len, temp);
            } else {
                break;
            }
        }
    }
    return max_len;
}

[
    [
        [1, 2, 3, 2, 1],
        [3, 2, 1, 4, 7],
    ],
    [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
    [
        [9, 8, 2, 5, 5, 7, 10, 7, 4, 2],
        [4, 3, 10, 7, 6, 7, 4, 5, 4, 7],
    ],
    [
        [9, 8, 2, 5, 5, 7, 10, 7, 4, 2],
        [4, 3, 10, 7, 6, 7, 2, 5, 5, 7, 10, 7],
    ],
    [
        [
            3, 2, 4, 3, 4, 4, 2, 1, 2, 2, 4, 3, 4, 1, 2, 3, 1, 3, 4, 3, 4, 2, 2, 3, 1, 4, 4, 3, 2,
            2, 1, 4, 4, 1, 4, 4, 4, 1, 3, 2, 2, 2, 2, 4, 3, 2, 3, 2, 3, 3, 4, 2, 2, 2, 2, 2, 4, 1,
            3, 3, 2, 2, 4, 3, 3, 2, 3, 2, 4, 4, 4, 3, 4, 2, 1, 3, 3, 4, 3, 1, 2, 2, 3, 2, 2, 1, 4,
            3, 1, 3, 3, 3, 4, 2, 4, 4, 2, 4, 1, 4,
        ],
        [
            1, 1, 1, 3, 4, 2, 3, 4, 2, 3, 3, 2, 1, 3, 1, 4, 3, 4, 2, 2, 1, 4, 1, 2, 3, 1, 4, 4, 3,
            3, 1, 3, 1, 2, 3, 3, 1, 1, 2, 4, 1, 2, 1, 2, 4, 3, 3, 1, 3, 3, 3, 2, 3, 1, 1, 4, 3, 3,
            2, 1, 3, 2, 2, 1, 4, 3, 2, 1, 1, 2, 2, 4, 1, 1, 2, 3, 1, 1, 2, 2, 4, 2, 4, 2, 2, 2, 2,
            2, 4, 2, 4, 3, 4, 2, 3, 2, 3, 2, 3, 2,
        ],
    ],
].forEach(([nums1, nums2]) => console.log(findLength(nums1, nums2)));
