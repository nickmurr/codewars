package main

import (
	"fmt"
	"strings"
)

func main() {
	test1 := CreatePhoneNumber1([10]uint{1, 2, 3, 4, 5, 6, 7, 8, 9, 0})
	fmt.Println(test1)
}

func CreatePhoneNumber1(numbers [10]uint) string {
	first := ""
	second := ""
	third := ""

	for i, a := range numbers {
		if i < 3 {
			first = first + fmt.Sprint(a)
		}

		if i > 2 && i < 6 {
			second = second + fmt.Sprint(a)
		}

		if i >= 6 {
			third = third + fmt.Sprint(a)
		}
	}

	return fmt.Sprintf("(%v) %v-%v", first, second, third)
}

func CreatePhoneNumber2(n [10]uint) string {
	return fmt.Sprintf("(%d%d%d) %d%d%d-%d%d%d%d", n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9])
}

func CreatePhoneNumber3(numbers [10]uint) string {
	var test string = strings.Trim(strings.Replace(fmt.Sprint(numbers), " ", "", -1), "[]")
	return fmt.Sprintf("(%s) %s-%s", test[0:3], test[3:6], test[6:10])
}

func CreatePhoneNumber4(numbers [10]uint) string {
	tmp := make([]interface{}, len(numbers))
	for i, val := range numbers {
		tmp[i] = val
	}
	return fmt.Sprintf("(%d%d%d) %d%d%d-%d%d%d%d", tmp...)
}
