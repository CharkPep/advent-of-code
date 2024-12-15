package main

import (
	"fmt"
	"strconv"
	"sync"
)

var (
	cached map[int]map[int]int = make(map[int]map[int]int)
	mux    *sync.Mutex         = &sync.Mutex{}
	N                          = 75
)

func dfs(n, level int) int {
	if level == N {
		return 1
	}

	if cached[level] == nil {
		cached[level] = make(map[int]int)
	}

	if v, ok := cached[level][n]; ok {
		return v
	}

	s := fmt.Sprintf("%d", n)
	switch {
	case len(s)%2 == 0:
		n1, _ := strconv.ParseUint(s[0:len(s)/2], 10, 64)
		n2, _ := strconv.ParseUint(s[len(s)/2:], 10, 64)
		r1 := dfs(int(n1), level+1)
		r2 := dfs(int(n2), level+1)
		mux.Lock()
		defer mux.Unlock()
		cached[level][n] = r1 + r2
		return r1 + r2
	case n == 0:
		r := dfs(1, level+1)
		mux.Lock()
		defer mux.Unlock()
		cached[level][n] = r
		return r
	default:
		r := dfs(n*2024, level+1)
		mux.Lock()
		defer mux.Unlock()
		cached[level][n] = r
		return r
	}

}

func main() {
	n := []int{20, 82084, 1650, 3, 346355, 363, 7975858, 0}
	ans := 0
	for _, v := range n {
		ans += dfs(v, 0)
	}

	fmt.Println(ans)
}
