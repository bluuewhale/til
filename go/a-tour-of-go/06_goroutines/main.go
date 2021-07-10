package main

import (
	"fmt"
	"time"

	"golang.org/x/tour/tree"
)

// Goroutines
func say(s string) {
	for i := 0; i < 5; i++ {
		time.Sleep(100 * time.Millisecond)
		fmt.Println(s)
	}
}

func testGoroutine() {
	fmt.Println("Goroutine")

	go say("world")
	say("hello")
}

// Channels
func sum(s []int, c chan int) {
	sum := 0
	for _, v := range s {
		sum += v
	}

	c <- sum // send sum to c
}

func testChannel() {
	fmt.Println("Channel")

	s := []int{7, 2, 8, -9, 4, 0}
	c := make(chan int)

	go sum(s[:len(s)/2], c)
	go sum(s[len(s)/2:], c)
	x, y := <-c, <-c // receive from c

	fmt.Println(x, y)
}

func testBufferedChannel() {
	fmt.Println("Buffered Channel")

	ch := make(chan int, 100)

	ch <- 1
	ch <- 2
	fmt.Println(<-ch)
	fmt.Println(<-ch)
}

func fibonacci(n int, c chan int) {
	x, y := 0, 1
	for i := 0; i < n; i++ {
		c <- x
		x, y = y, x+y
	}

	close(c)
}

func testRangeWithChannel() {
	// Note: Only the sender should close a channel, never the receiver.
	// Sending on a closed channel will cause a panic.

	// Another note: Channels aren't like files;
	// you don't usually need to close them. Closing is only necessary
	// when the receiver must be told there are no more values coming,
	// such as to terminate a range loop.
	fmt.Println("Range and Close")

	c := make(chan int, 10)
	go fibonacci(cap(c), c)
	for i := range c {
		fmt.Println(i)
	}
}

func fibonacciWithSelect(quit chan int, c chan int) {
	x, y := 0, 1
	for {
		select {
		case c <- x:
			x, y = y, x+y
		case <-quit:
			fmt.Println("quit!")
			return
		}
	}
}

func testChannelWithSelect() {
	fmt.Println("Select")

	c := make(chan int)
	quit := make(chan int)

	go func() {
		for i := 0; i < 10; i++ {
			c <- i
			fmt.Println(<-c)
		}

		quit <- 0
	}()

	time.Sleep(3 * time.Second)
	fibonacciWithSelect(c, quit)
}

func testChannelWithDefaultSelection() {
	tick := time.Tick(100 * time.Millisecond)
	boom := time.After(500 * time.Millisecond)

	for {
		select {
		case <-tick:
			fmt.Println("tick. ")
		case <-boom:
			fmt.Println("BOOM!")
			return
		default:
			fmt.Println(".")
			time.Sleep(50 * time.Millisecond)
		}
	}
}

func traverseInorder(t *tree.Tree, result *[]int) {
	if t == nil {
		return
	}

	traverseInorder(t.Left, result)
	*result = append(*result, t.Value)
	traverseInorder(t.Right, result)
}

func TraverseInorder(t *tree.Tree) []int {
	result := []int{}
	traverseInorder(t, &result)
	return result
}

func Walk(t *tree.Tree, ch chan int) {
	for _, v := range TraverseInorder(t) {
		ch <- v
	}

	close(ch)
}

func IsSame(t1, t2 *tree.Tree) bool {
	ch1 := make(chan int)
	ch2 := make(chan int)

	go Walk(t1, ch1)
	go Walk(t2, ch2)

	for {
		v1, ok1 := <-ch1
		v2, ok2 := <-ch2

		if !ok1 || !ok2 {
			return true
		}

		if v1 != v2 {
			// value does not match
			return false
		}
	}
}

func exerciseEquivalentBinaryTrees() {
	fmt.Println("Exercise: Equivalent Binary Trees")

	fmt.Println(IsSame(tree.New(1), tree.New(1)))
	fmt.Println(IsSame(tree.New(1), tree.New(2)))
}

func main() {
	//testGoroutine()
	//testChannel()
	//testBufferedChannel()
	//testRangeWithChannel()
	//testChannelWithSelect()
	//testChannelWithDefaultSelection()
	exerciseEquivalentBinaryTrees()
}
