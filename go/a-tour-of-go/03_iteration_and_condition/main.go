package main

import (
	"fmt"
	"math"
	"runtime"
	"time"
)

func sqrt(x float64) string {
	if x < 0 {
		return sqrt(-x) + "i"
	}
	return fmt.Sprint(math.Sqrt(x))
}

func Sqrt(x float64) float64 {
	z := x
	for i := 0; i < 10; i++ {
		z -= (z*z - x) / (2 * z)
	}
	return z
}

func main() {
	var sum int = 0
	for i := 0; i < 10; i++ {
		sum += i
	}

	fmt.Println(sum)               // 45
	fmt.Println(sqrt(2), sqrt(-4)) // 1.41421, 2i

	// loop exercise
	fmt.Println(Sqrt(2)) // 1.41421

	// switch
	var os string = runtime.GOOS
	switch os {
	case "darwin":
		fmt.Println("OS X")
	case "linux":
		fmt.Println("Linux")
	default:
		fmt.Printf("%s \n", os)
	}

	var t time.Time = time.Now()
	switch {
	case t.Hour() < 12:
		fmt.Println("Good morning!")
	case t.Hour() < 17:
		fmt.Println("Good afternoon!")
	default:
		fmt.Println("Good evening!")
	}

	// defer
	// defer statement defers the execution of a function
	// until the surrounding function returns
	defer fmt.Println("world!")
	fmt.Printf("hello ") // hello world!

}
