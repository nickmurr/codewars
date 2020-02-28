package main

import (
	"fmt"
	"math"
)

func main() {
	times1 := BouncingBall1(3, 0.66, 1.5)
	times2 := BouncingBall1(40, 0.4, 10)
	times3 := BouncingBall1(98, 0.66, 9.8)
	times4 := BouncingBall1(2, 0.5, 1)

	fmt.Println(times1, times2, times3, times4)

}

func BouncingBall1(h, bounce, window float64) (times int) {
	if h > 0 && bounce > 0 && bounce < 1 && window < h {
		times = -1
		for h > window {
			times += 2
			h *= bounce
		}
		return times
	}
	return -1
}

func BouncingBall2(h, bounce, window float64) int {
	if h < 0 || bounce <= 0 || 1 <= bounce || h < window {
		return -1
	}

	var count int = -1
	for ; h > window; h *= bounce {
		count += 2
	}

	return count
}

func BouncingBall3(h, bounce, window float64) int {
	if h <= window || bounce <= 0 || bounce >= 1 {
		return -1
	} else {
		return 2 + BouncingBall3((h*bounce), bounce, window)
	}
}

func BouncingBall4(h, bounce, window float64) int {
	if !(h > 0 && 0 < bounce && bounce < 1 && window < h) {
		return -1
	}
	return int(math.Ceil(math.Log(window/h)/math.Log(bounce)))*2 - 1
}
