package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
	"unicode"
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
	stringMap := strings.Map(func(r rune) rune {
		return caesar(r, shift)
	}, strings.ToLower(s))
	
	return CaseSensetive(stringMap)

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

func CaseSensetive(str string) string {
	out := []string{}
	x := strings.Split(str, "")

	for i, v := range x {
		if i%2 == 0 {
			out = append(out, strings.ToUpper(v))
			continue
		}
		out = append(out, strings.ToLower(v))
	}

	return Reverse(strings.Join(out, ""))
}


func transform(r rune, shift int32) rune {
	if unicode.IsLower(r) { r = 'a' + (r - 'a' + shift) % 26 }
	if unicode.IsUpper(r) { r = 'A' + (r - 'A' + shift) % 26 }
	if unicode.IsDigit(r) { r = '9' - r + '0' }
	return r
}

func PlayPass2(s string, n int) string {
	// Step 1
	p := ""
	for i, r := range s {
		r = transform(r, int32(n))
		if i % 2 == 0 && unicode.IsLetter(r) {
			r = unicode.ToUpper(r)
		} else {
			r = unicode.ToLower(r)
		}
		p = string(r) + p
	}

	return p
}
