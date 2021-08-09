package main

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestEmptyContext(t *testing.T) {
	ctx := context.Background() // Empty context will never be cancelled nor done

	done := ctx.Done()
	if done != nil {
		panic(done)
	}

	err := ctx.Err()
	if err != nil {
		panic(err)
	}

	deadline, ok := ctx.Deadline()
	if ok {
		panic(deadline)
	}
}

func TestWithCancel(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	start := time.Now()
	go func() {
		time.Sleep(time.Second)
		cancel()
	}()

	<-ctx.Done()
	assert.Greater(t, time.Since(start).Seconds(), float64(1))
}

func TestWithTimeout(t *testing.T) {
	start := time.Now()

	ctx, _ := context.WithTimeout(context.Background(), time.Second)

	<-ctx.Done()
	assert.Greater(t, time.Since(start).Seconds(), float64(1))
}

func TestWithDeadline(t *testing.T) {
	start := time.Now()
	expire := start.Add(time.Second)
	ctx, _ := context.WithDeadline(context.Background(), expire)

	<-ctx.Done()
	assert.Greater(t, time.Since(start).Seconds(), float64(1))
}

func TestRunFunctionWithContext(t *testing.T) {

	myFunc := func() int {
		time.Sleep(time.Second * 3)
		return 1
	}

	myFuncWithCtx := func(ctx context.Context) (int, error) {
		done := make(chan int)

		go func() {
			done <- myFunc()
		}()

		select {
		case result := <-done:
			return result, nil
		case <-ctx.Done():
			return 0, ctx.Err()
		}
	}

	_, err := myFuncWithCtx(context.Background()) // this should return 1 with no error
	if err != nil {
		panic(err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	go func() {
		time.Sleep(time.Second)
		cancel()
	}()

	_, err = myFuncWithCtx(ctx) // this should return 0 with no CancelError after 1 second
	if err == nil {
		panic(err)
	}
}
