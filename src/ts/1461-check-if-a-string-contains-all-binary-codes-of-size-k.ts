// 1461. Check If a String Contains All Binary Codes of Size K
// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/

function hasAllCodes(s: string, k: number): boolean {
    const hashSetLen = 2 ** k;
    if (s.length < k + hashSetLen - 1) {
        return false;
    }
    let lastHash = hash(s.slice(0, k));
    const hashSet = new Set([lastHash]);
    for (let i = 0; i < s.length - k; i++) {
        lastHash = rollingHash(lastHash, k, s[i], s[i + k]);
        hashSet.add(lastHash);
        if (hashSet.size === hashSetLen) {
            return true;
        }
    }
    return false;
}

const hash = (s: string) =>
    [...Array(s.length).keys()].reduce(
        (sum, i) => sum + parseInt(s[i]) * (1 << (s.length - i - 1)),
        0
    );

const rollingHash = (lastHash: number, k: number, prev: string, next: string) =>
    ((lastHash - parseInt(prev) * (1 << (k - 1))) << 1) + parseInt(next[0]);

console.log(hasAllCodes('00110110', 2));
console.log(hasAllCodes('0110', 1));
console.log(hasAllCodes('0110', 2));
console.log(hasAllCodes('00110', 2));
