package main

import (
	"fmt"
	"math"
)

func main() {
	test1 := SquaresInRect(5, 3)
	test2 := SquaresInRect(20, 14)
	test3 := SquaresInRect(8, 4)
	fmt.Println(test1, test2, test3)
}

func SquaresInRect(lng int, wdth int) []int {

	if lng == wdth {
		return nil
	}

	// your code
	total := lng * wdth
	// init := int(math.Floor(math.Sqrt(float64(total))))
	init := int((math.Floor(math.Sqrt(float64(total)))))
	out := make([]int, 0)

	for i := init; total > 0; {
		if i == 1 && total > 0 {
			out = append(out, 1)
			total -= 1
			continue
		}
		if i > lng || i > wdth || i*i > total {
			i--
			continue
		}
		total -= i * i
		out = append(out, i)

		if !(total%i == 0) {
			i--
		}
	}

	return out
}
