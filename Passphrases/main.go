package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func main() {

	a := PlayPass("AAABBCCY", 1)
	b := PlayPass("I LOVE YOU!!!", 1)
	c := PlayPass("I LOVE YOU!!!!", 1)
	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)

}

func PlayPass(s string, shift int) string {
	r := strings.Map(func(r rune) rune {
		return caesar(r, shift)
	}, strings.ToLower(s))

	x := strings.Split(r, "")
	out := []string{}
	for i, v := range x {
		// match, _ := regexp.Match("/[a-zA-Z]/", []byte(v))
		if i%2 == 0 {
			out = append(out, strings.ToUpper(v))
			continue
		}
		out = append(out, strings.ToLower(v))

	}
	return Reverse(strings.Join(out, ""))
}

func caesar(r rune, shift int) rune {
	stringMatch, _ := regexp.Match("[a-zA-Z]", []byte{byte(r)})
	numMatch, _ := regexp.Match("[0-9]", []byte{byte(r)})

	if stringMatch {
		s := int(r) + shift
		if s > 'z' {
			return rune(s - 26)
		} else if s < 'a' {
			return rune(s + 26)
		}
		return rune(s)
	}

	if numMatch {
		atoi, _ := strconv.Atoi(string(r))
		return rune(9 - atoi + 26 + 22)
	}

	return r
}

func Reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}
