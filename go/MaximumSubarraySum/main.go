package main

import (
	"fmt"
	"math"
)

func main() {
	test1 := GetMaxSum2([]int{-2, 1, -3, 4, -1, 2, 1, -5, 4})
	test2 := GetMaxSum2([]int{-2, -1, -3, -4, -1, -2, -1, -5, -4})
	fmt.Println(test1,test2)
}

func max(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}
func GetMaxSum2(numbers []int) int {
	ans, sum := 0, 0

	for i := 0; i < len(numbers); i++ {
		sum += numbers[i]
		ans = max(ans, sum)
		sum = max(sum, 0)
	}

	return ans
}

func maximumSubarraySum(numbers []int) int {
	max, subMax := 0., 0.

	for _, n := range numbers {
		subMax = math.Max(subMax+float64(n), 0.)
		max = math.Max(subMax, max)
	}

	return int(max)
}
