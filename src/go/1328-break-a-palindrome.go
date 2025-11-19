// 1328. Break a Palindrome
// https://leetcode.com/problems/break-a-palindrome/

package main

import (
	"fmt"
	"strings"
)

func breakPalindrome(palindrome string) string {
	if len(palindrome) == 1 {
		return ""
	}
	index := strings.IndexFunc(palindrome[:len(palindrome)/2], func(c rune) bool { return c != 'a' })
	if index != -1 {
		return palindrome[:index] + "a" + palindrome[index+1:]
	}
	return palindrome[:len(palindrome)-1] + "b"
}

func main() {
	for _, palindrome := range []string{"abccba", "a", "aaaaa", "aba", "bcb", "aabaa"} {
		fmt.Println(breakPalindrome(palindrome))
	}
}
