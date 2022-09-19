// 609. Find Duplicate File in System
// https://leetcode.com/problems/find-duplicate-file-in-system/

package main

import (
	"fmt"
	"hash/fnv"
	"strings"
)

func findDuplicate(paths []string) [][]string {
	m := make(map[uint32][]string)
	for _, path := range paths {
		split := strings.Split(path, " ")
		prefix := split[0]
		for _, file := range split[1:] {
			file_split := strings.Split(file, "(")
			content := file_split[1]
			content_hash := hash(content[:len(content)-1])
			full_path := strings.Join([]string{prefix, file_split[0]}, "/")
			if val, ok := m[content_hash]; ok {
				m[content_hash] = append(val, full_path)
			} else {
				m[content_hash] = []string{full_path}
			}
			fmt.Println("=>", m)
		}
	}
	res := make([][]string, 0, len(m))
	for _, val := range m {
		if len(val) > 1 {
			res = append(res, val)
		}
	}
	return res
}

func hash(s string) uint32 {
	h := fnv.New32a()
	h.Write([]byte(s))
	return h.Sum32()
}

func main() {
	for _, paths := range [][]string{
		{"root/a 1.txt(abcd) 2.txt(efgh)", "root/c 3.txt(abcd)", "root/c/d 4.txt(efgh)", "root 4.txt(efgh)"},
		{"root/a 1.txt(abcd) 2.txt(efgh)", "root/c 3.txt(abcd)", "root/c/d 4.txt(efgh)"},
		{"root/a 1.txt(abcd) 2.txt(efsfgh)", "root/c 3.txt(abdfcd)", "root/c/d 4.txt(efggdfh)"},
	} {
		fmt.Println(findDuplicate(paths))
	}
}
