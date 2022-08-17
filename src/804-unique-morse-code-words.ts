// 804. Unique Morse Code Words
// https://leetcode.com/problems/unique-morse-code-words/

function uniqueMorseRepresentations(words: string[]): number {
    return new Set(words.map(transform)).size;
}

// prettier-ignore
const MORSE_CODES = ['.-','-...','-.-.','-..','.','..-.','--.','....','..','.---','-.-','.-..','--','-.','---','.--.','--.-','.-.','...','-','..-','...-','.--','-..-','-.--','--..'];

const transform = (word: string) =>
    [...Array(word.length).keys()].map((i) => MORSE_CODES[word.charCodeAt(i) - 97]).join('');

console.log([['gin', 'zen', 'gig', 'msg'], ['a']].map(uniqueMorseRepresentations));
