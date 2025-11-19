// 387. First Unique Character in a String
// https://leetcode.com/problems/first-unique-character-in-a-string/

const INIT_CHAR_CODE = 'a'.charCodeAt(0);

function firstUniqChar(s: string): number {
    const count: number[] = Array(26).fill(0);
    const pos: number[] = Array(26).fill(-1);
    for (let i = 0; i < s.length; i++) {
        const c = s.charCodeAt(i) - INIT_CHAR_CODE;
        count[c]++;
        if (pos[c] === -1) pos[c] = i;
    }
    const uniqueChars = count
        .map((val, i) => [val, i])
        .filter((val) => val[0] === 1)
        .map((val) => pos[val[1]]);
    return uniqueChars.length !== 0 ? Math.min(...uniqueChars) : -1;
}

console.log(
    [
        'leetcode',
        'loveleetcode',
        'aabb',
        'ofderjamodcvbpuuwlrnqrkmkmeynmfrhhwvfnfjextwvlbswbfcmzfdqhja',
    ].map(firstUniqChar)
);
