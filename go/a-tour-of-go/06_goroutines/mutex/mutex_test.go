package mutex_test

import (
	"fmt"
	"mutex"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMutex(t *testing.T) {
	sc := mutex.NewSafeCounter()

	done := make(chan bool)

	go func() {
		for i := 0; i < 10; i++ {
			fmt.Println(i)
			go sc.Increase("hi")
		}

		done <- true
	}()

	if d := <-done; d {
		assert.True(t, d, "expected true, but got false")
	}
}
