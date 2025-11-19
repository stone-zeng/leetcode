// 820. Short Encoding of Words
// https://leetcode.com/problems/short-encoding-of-words/

type Flag = 'Skip' | 'Replace' | 'Push';

function minimumLengthEncoding(words: string[]): number {
    const reference = [words[0]];
    for (const word of words.slice(1)) {
        let flag: Flag = 'Push';
        let index: number = 0;
        for (let i = 0; i < reference.length; i++) {
            const r = reference[i];
            if (r.endsWith(word)) {
                flag = 'Skip';
                break;
            } else if (word.endsWith(r)) {
                flag = 'Replace';
                index = i;
                break;
            }
        }
        if (flag === 'Replace') {
            reference[index] = word;
        } else if (flag === 'Push') {
            reference.push(word);
        }
    }
    return reference.reduce((sum, s) => sum + s.length + 1, 0);
}

console.log([['time', 'me', 'bell'], ['t']].map(minimumLengthEncoding));
