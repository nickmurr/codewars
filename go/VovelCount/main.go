package main

import (
	"fmt"
	"strings"
)

func main() {
	a := GetCount1("abracadabra")
	b := GetCount2("abracadabraabracadabra")

	fmt.Println(a, b)
}

func GetCount1(str string) (count int) {
	vowels := []string{"a", "e", "i", "o", "u"}
	for _, v := range vowels {
		count += strings.Count(str, v)
	}
	return count
}

func GetCount2(str string) (count int) {
	for _, c := range str {
		switch c {
		case 'a', 'e', 'i', 'o', 'u':
			count++
		}
	}
	return count
}

func GetCount3(str string) (count int) {
	m := map[byte]int{
		'a': 1,
		'e': 1,
		'i': 1,
		'o': 1,
		'u': 1,
	}
	for i := range str {
		count += m[str[i]]
	}
	return count
}
