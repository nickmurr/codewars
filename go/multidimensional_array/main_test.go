package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_fill(t *testing.T) {
	type args struct {
		x     int
		y     int
		limit int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "LENGTH",
			args: args{
				x:     5,
				y:     5,
				limit: 100,
			},
			want: 5,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			res := fill(tt.args.x, tt.args.y, tt.args.limit)
			assert.Equal(t, tt.want, len(res))
		})
	}
}

func Benchmark_fill(b *testing.B) {
	for i := 0; i < b.N; i++ {
		fill(10, 10, 1000)
	}
}
func Benchmark_fillAppend(b *testing.B) {
	for i := 0; i < b.N; i++ {
		fillAppend(10, 10, 1000)
	}
}

//  go test ./... -bench=. -benchmem
