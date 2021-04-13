package main

import (
	"fmt"
	"math"
)

var words = []string{"zero", "one", "two", "three", "four",
	"five", "six", "seven", "eight", "nine",
	"ten", "eleven", "twelve", "thirteen",
	"fourteen", "quarter", "sixteen", "seventeen",
	"eighteen", "nineteen", "twenty", "twenty one",
	"twenty two", "twenty three", "twenty four",
	"twenty five", "twenty six", "twenty seven",
	"twenty eight", "twenty nine", "half",
}

func suffix(i int32) string {
	switch i {
	case 15:
		return "quarter"
	case 30:
		return "half"
	case 1, 21:
		return fmt.Sprintf("%s minute", words[i])
	default:
		return fmt.Sprintf("%s minutes", words[i])
	}
}

// Complete the timeInWords function below.
func timeInWords(h int32, m int32) string {
	if m == 0 {
		return fmt.Sprintf("%s o' clock", words[h])
	}
	if 1 <= m && m <= 30 {
		return fmt.Sprintf("%s past %s", suffix(m), words[h])
	}
	if m > 30 {
		return fmt.Sprintf("%s to %s", suffix(int32(math.Abs(float64(m)-60))), words[h+1])
	}

	return ""
}
