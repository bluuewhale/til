package mutex

import (
	"sync"
)

type SafeCounter struct {
	mx    sync.Mutex
	table map[string]int
}

func NewSafeCounter() SafeCounter {
	return SafeCounter{mx: sync.Mutex{}, table: make(map[string]int)}
}

func (SafeCounter) New() SafeCounter {
	return SafeCounter{mx: sync.Mutex{}, table: make(map[string]int)}
}

func (sc *SafeCounter) Get(key string) int {
	sc.mx.Lock()
	if _, found := sc.table[key]; !found {
		sc.table[key] = 0
	}
	defer sc.mx.Unlock()
	return sc.table[key]
}

// Increase the counter for given key
func (sc *SafeCounter) Increase(key string) {
	sc.mx.Lock()
	sc.table[key] = sc.Get(key) + 1
	sc.mx.Unlock()
}
