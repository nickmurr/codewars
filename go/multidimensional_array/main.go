package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	fmt.Println(fill(5, 5, 100))
}

func fill(x, y, limit int) [][]int {
	arr := make([][]int, x)
	rand.Seed(time.Now().UnixNano())

	for i := 0; i < x; i++ {
		n := make([]int, y)
		for j := 0; j < y; j++ {
			n[j] = rand.Intn(limit)
		}
		arr[i] = n
	}

	return arr
}

func fillAppend(x, y, limit int) [][]int {
	var arr [][]int
	rand.Seed(time.Now().UnixNano())

	for i := 0; i < x; i++ {
		var n []int
		for j := 0; j < y; j++ {
			n = append(n, rand.Intn(limit))
		}
		arr = append(arr, n)
	}

	return arr
}
