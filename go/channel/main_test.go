package main

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestSimpleChannel(t *testing.T) {
	ch := make(chan int)

	go func() {
		ch <- 1
	}()

	result := <-ch
	assert.Equal(t, 1, result, "Error: TestSimpleChannel()")
}

func TestSyncChannel(t *testing.T) {

	start := time.Now()
	done := make(chan bool)

	go func() {
		time.Sleep(time.Second)
		done <- true
	}()

	<-done

	assert.Greater(t, time.Since(start).Seconds(), float64(1))
}

func TestBufferedChannel(t *testing.T) {
	ch := make(chan int, 5)
	count := 5

	go func() {
		for i := 0; i < count; i++ {
			ch <- i
			time.Sleep(time.Second)
		}
	}()

	start := time.Now()
	for i := 0; i < count; i++ {
		<-ch // this be called only when buffer is full or sender is closed
	}
	assert.Greater(t, time.Since(start).Seconds(), float64(1.1))
}

func TestProducerConsumer(t *testing.T) {
	ch := make(chan int)

	produce := func(c chan<- int) {
		for i := 0; i < 5; i++ {
			time.Sleep(time.Second)
			c <- i
		}
	}

	consume := func(c <-chan int) {
		for i := 0; i < 5; i++ {
			<-c
		}
	}

	start := time.Now()

	go produce(ch)
	consume(ch)

	assert.Greater(t, time.Since(start).Seconds(), float64(5))
}

func TestChannelWithSelect(t *testing.T) {
	ch1 := make(chan int)
	ch2 := make(chan int)

	go func() {
		ch1 <- 1
	}()

	time.Sleep(time.Second)

	select {
	case <-ch1:
	case <-ch2:
		assert.Fail(t, "unexpected")
	default:
		assert.Fail(t, "unexpected")
	}

	// ==================================================

	ch3 := make(chan int)
	ch4 := make(chan int)

	time.Sleep(time.Second)

	select {
	case <-ch3:
		assert.Fail(t, "unexpected")
	case <-ch4:
		assert.Fail(t, "unexpected")
	default:
	}
}
