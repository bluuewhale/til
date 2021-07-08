// Every Go program is made up of packages

package main

import (
	"fmt"
	"math"
	"math/rand"
)

func add(x int, y int) int {
	// return sum of x and y
	return x + y
}

func swap(x string, y string) (string, string) {
	// swap x and y
	return y, x
}

func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return
}

func main() {
	fmt.Println("My favorite number is ", rand.Intn(100))
	fmt.Println("Pi is ", math.Pi)

	// function and variables
	fmt.Println("3 + 4 = ", add(3, 4))

	var x string = "hello"
	var y string = "world"
	fmt.Println(swap(x, y))

	fmt.Println("(3 , 4) = ", add(3, 4))
	fmt.Println(split(17))

	// uninitialized variables
	var c, python, java bool
	var i int
	fmt.Println(i, c, python, java, x)

	// implicit declaration
	k := 3
	fmt.Println(k)

	// constants
	const Pi string = "3.141592"
	fmt.Println(Pi)

}
