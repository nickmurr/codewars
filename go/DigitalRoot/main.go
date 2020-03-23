package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	test1 := DigitalRoot1(10203098)
	fmt.Println(test1)
}

func DigitalRoot1(n int) int {
	init := strings.SplitAfter(fmt.Sprint(n), "")
	sum := 0

	for _, v := range init {
		num, _ := strconv.Atoi(v)
		sum += num
	}

	if len(fmt.Sprint(sum)) > 1 {
		return DigitalRoot1(sum)
	}

	return sum
}

func getSum(n int) int {
	if n > 0 && n < 10 {
		return n
	} else if n <= 0 {
		return 0
	}

	return getSum(n/10) + getSum(n%10)
}

func DigitalRoot2(n int) int {
	if n >= 0 && n < 10 {
		return n
	}
	return DigitalRoot2(getSum(n))
}

func DigitalRoot3(n int) int {
	return (n-1)%9 + 1
}
