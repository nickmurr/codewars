package main

import "testing"

func Test_timeInWords(t *testing.T) {
	type args struct {
		h int32
		m int32
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "5:00",
			args: args{5, 0},
			want: "five o' clock",
		},
		{
			name: "5:01",
			args: args{5, 01},
			want: "one minute past five",
		},
		{
			name: "5:30",
			args: args{5, 30},
			want: "half past five",
		},
		{
			name: "5:45",
			args: args{5, 45},
			want: "quarter to six",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := timeInWords(tt.args.h, tt.args.m); got != tt.want {
				t.Errorf("timeInWords() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Benchmark_timeInWords_parallel(b *testing.B) {
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			timeInWords(5, 01)
		}
	})
}
func Benchmark_timeInWords(b *testing.B) {
	for i := 0; i < b.N; i++ {
		timeInWords(5,01)
	}
}
