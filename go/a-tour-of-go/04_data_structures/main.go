package main

import (
	"fmt"
	"math"
	"strings"

	"golang.org/x/tour/wc"
)

func test_pointers() {
	var i int = 42

	// referencing
	var p1 *int = &i
	fmt.Println(p1, *p1) // 0x...., 42

	// deferencing
	*p1 = 21
	println(i) // 21
}

func test_struct() {
	// Struct is a collection of fields
	type Position struct {
		X int
		Y int
	}

	var pos Position = Position{1, 2}
	pos.X = 3
	fmt.Println(pos) // {3 2}

	var ptr *Position = &pos
	ptr.Y = 4
	fmt.Println(pos) // {3 4}
}

func printSlice(s []int) {
	fmt.Printf("len=%d, cap=%d %v\n", len(s), cap(s), s)
}

func test_array() {
	var a [2]string
	a[0] = "Hello"
	a[1] = "World"
	fmt.Println(a[0], a[1]) // Hello World
	fmt.Println(a)          // [Hello World]

	// declaration + initialization
	var primes [6]int = [6]int{2, 3, 5, 7, 11, 13}
	printSlice(primes[:])

	// slices
	var slice []int = primes[1:4]
	printSlice(slice)

	// nil slice
	var s []int
	printSlice(s)
	if s == nil {
		fmt.Println("nil!")
	}

}

func test_make() {
	// make
	array1 := make([]int, 5)
	fmt.Println("array1", array1, len(array1), cap(array1))

	array2 := make([]int, 0, 5)
	fmt.Println("array2", array2, len(array2), cap(array2))

	array3 := array2[:2]
	fmt.Println("array3", array3, len(array3), cap(array3))

	array4 := array3[2:5]
	fmt.Println("array4", array4, len(array4), cap(array4))
}

func test_slice_of_slice() {
	// slice of slice
	board := [][]string{
		[]string{"-", "-", "-"},
		[]string{"-", "-", "-"},
		[]string{"-", "-", "-"},
	}

	board[0][0] = "X"
	board[2][2] = "O"
	board[1][2] = "X"
	board[1][0] = "O"
	board[0][2] = "X"

	for i := 0; i < len(board); i++ {
		fmt.Printf("%s\n", strings.Join(board[i], ""))
	}
}

func test_appending_to_a_slice() {
	var s []int
	printSlice(s)

	// append works on nil slices
	s = append(s, 0)
	printSlice(s)

	// the slice grows as needed
	s = append(s, 1)
	printSlice(s)

	// we can add more than one element at a time
	s = append(s, 2, 3, 4)
	printSlice(s)
}

func test_range() {
	var pow = []int{1, 2, 4, 8, 16, 32, 64, 128, 256}
	for idx, val := range pow {
		fmt.Printf("2**%d = %d\n", idx, val)
	}
}

func exercies_arrays(dx int, dy int) [][]uint8 {
	var result = make([][]uint8, dx)

	for x := 0; x < dx; x++ {
		result[x] = make([]uint8, dy)

		for y := 0; y < dy; y++ {
			result[x][y] = uint8((x + y) / 2)
		}
	}

	return result
}

type Vertex struct {
	Lat  float64
	Long float64
}

func test_maps() {
	var m = make(map[string]Vertex)
	m["Bell Labs"] = Vertex{
		40.68433, -74.39967,
	}

	fmt.Println(m["Bell Labs"])
}

func test_map_literal() {
	var m = map[string]Vertex{
		"Bell Labs": Vertex{
			40.68433, -74.39967,
		},

		"Google": {
			37.42202, -122.08408,
		},
	}
	fmt.Println(m)
}

func test_mutating_maps() {
	m := make(map[string]int)

	m["answer"] = 42
	fmt.Printf("The answer is %d \n", m["answer"])

	m["answer"] = 21
	fmt.Printf("The answer is %d \n", m["answer"])

	delete(m, "answer")
	fmt.Printf("The answer is %d \n", m["answer"])

	v, ok := m["answer"]
	fmt.Printf("The answer is %d. is it ok? %v", v, ok)
}

func exercise_maps(s string) map[string]int {
	var wcMap = make(map[string]int)
	var words = strings.Fields(s)

	for _, w := range words {
		_, ifContains := wcMap[w]
		if ifContains {
			wcMap[w] += 1
		} else {
			wcMap[w] = 1
		}
	}

	return wcMap
}

func compute(fn func(float64, float64) float64) float64 {
	return fn(3, 4)
}

func test_function_values() {
	// functions are values too.
	var add = func(x float64, y float64) float64 {
		return x + y
	}
	fmt.Println("3 + 4 = ", compute(add))
	fmt.Println("3 ** 4 = ", compute(math.Pow))

}

func fibonacci() func() int {
	var i = 0
	var p int
	var v int

	return func() int {
		var result int

		switch {
		case i == 0:
			result = 0
		case i == 1:
			result = 1
			p = 0
			v = 1
		default:
			result = p + v
			p = v
			v = result
		}

		i += 1
		return result
	}
}

func exercise_closure() {
	f := fibonacci()
	for i := 0; i < 10; i++ {
		fmt.Println(f())
	}

}

func main() {
	test_pointers()
	test_struct()
	test_array()
	test_make()
	test_slice_of_slice()
	test_appending_to_a_slice()
	test_range()
	//pic.Show(exercise_arrays)

	test_maps()
	test_map_literal()
	test_mutating_maps()
	wc.Test(exercise_maps)

	test_function_values()
	exercise_closure()
}
