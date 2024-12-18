package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

// modify these as needed
const DEBUG_DIM = 7
const ACTUAL_DIM = 71
const ROWS = ACTUAL_DIM
const COLS = ACTUAL_DIM

// use this domain input for now
// i have a feeling we want the shortest path while the walls fall
const EMPTY = -1
const VISITED = -2
const NUM_BYTES = 1024

type Point struct {
	r, c, steps, priority int
}

type PriorityQueue []*Point

func (pq PriorityQueue) Len() int            { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool  { return pq[i].priority < pq[j].priority }
func (pq PriorityQueue) Swap(i, j int)       { pq[i], pq[j] = pq[j], pq[i] }
func (pq *PriorityQueue) Push(x interface{}) { *pq = append(*pq, x.(*Point)) }
func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[:n-1]
	return item
}

func manhattan(r int, c int) int {
	return int(math.Abs(float64(r-ROWS)) + math.Abs(float64(c-COLS))) // Manhattan distance
}

func main() {
	var maze [ROWS][COLS]int
	// -1 initialize
	for i := 0; i < ROWS; i++ {
		for j := 0; j < COLS; j++ {
			maze[i][j] = EMPTY
		}
	}

	file, err := os.Open("../input.in")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	tick := 0
	for scanner.Scan() {
		// if tick == NUM_BYTES {
		// 	break
		// }
		line := scanner.Text()
		parts := strings.Split(line, ",")
		c, _ := strconv.Atoi(parts[0])
		r, _ := strconv.Atoi(parts[1])
		maze[r][c] = tick
		tick++
		if ok, _ := search(maze); !ok {
			fmt.Printf("Tick:%d, (%d,%d)\n", tick, c, r)
			break
		}
	}

}

var DXNS = [4][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

// a* search
func search(maze [ROWS][COLS]int) (bool, int) {
	directions := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}} // Right, Down, Left, Up

	pq := &PriorityQueue{}
	heap.Init(pq)
	heap.Push(pq, &Point{r: 0, c: 0, steps: 0, priority: manhattan(0, 0)})

	visited := make(map[[2]int]bool)

	for pq.Len() > 0 {
		current := heap.Pop(pq).(*Point)
		if current.r == ROWS-1 && current.c == COLS-1 {
			return true, current.steps
		}

		if visited[[2]int{current.r, current.c}] {
			continue
		}
		visited[[2]int{current.r, current.c}] = true

		for _, delta := range directions {
			new_r, new_c := current.r+delta[0], current.c+delta[1]
			if new_r >= 0 && new_r < ROWS && new_c >= 0 && new_c < COLS && maze[new_r][new_c] < 0 {
				if visited[[2]int{new_r, new_c}] {
					continue
				}
				newCost := current.steps + 1
				heap.Push(pq, &Point{
					r:        new_r,
					c:        new_c,
					steps:    newCost,
					priority: newCost + manhattan(new_r, new_c),
				})
			}
		}
	}
	return false, 0
}
