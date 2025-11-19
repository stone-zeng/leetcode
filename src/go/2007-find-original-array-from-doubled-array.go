// 2007. Find Original Array From Doubled Array
// https://leetcode.com/problems/find-original-array-from-doubled-array/

package main

import (
	"fmt"
	"sort"
)

func findOriginalArray(changed []int) []int {
	if len(changed)%2 == 1 {
		return []int{}
	}
	sort.Sort(sort.IntSlice(changed))
	res := []int{}
	for len(changed) > 0 {
		i := changed[0]
		res = append(res, i)
		if i == 0 {
			if changed[1] == 0 {
				changed = changed[2:]
			} else {
				return []int{}
			}
		} else {
			j := sort.SearchInts(changed, i*2)
			if j < len(changed) && changed[j] == i*2 {
				changed = append(changed[1:j], changed[j+1:]...)
			} else {
				return []int{}
			}
		}
	}
	return res
}

func main() {
	for _, changed := range [][]int{
		{1, 3, 4, 2, 6, 8},
		{6, 3, 0, 1},
		{6, 3, 0, 0, 1, 2},
		{0, 0, 0, 0},
		{3, 3, 3, 3},
		{1, 3},
		{2, 3},
		{1},
		{18, 8, 9, 42, 50, 90, 17, 45, 39, 16, 32, 24, 40, 24, 36, 43, 100, 17, 48, 34, 74, 6, 78, 3, 9, 72, 24, 86, 84, 17, 4, 34, 12, 37, 18, 72, 12, 34, 36, 20},
	} {
		fmt.Println(findOriginalArray(changed))
	}
}
