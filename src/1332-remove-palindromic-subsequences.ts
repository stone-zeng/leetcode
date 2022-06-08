// 1332. Remove Palindromic Subsequences
// https://leetcode.com/problems/remove-palindromic-subsequences/

function removePalindromeSub(s: string): number {
    return isPalindrome(s) ? 1 : 2;
}

function isPalindrome(s: string): boolean {
    console.log([s.slice(0, s.length / 2), s.slice(s.length / 2 + 1)]);
    for (let i = 0; i < s.length / 2 - 1; i++) {
        if (s[i] !== s[s.length - i - 1]) return false;
    }
    return true;
}

console.log(
    ['a', 'ababa', 'abb', 'baabb', 'abababaabbbabbaabbbbabaaabaabbbabbbb'].map(removePalindromeSub)
);
