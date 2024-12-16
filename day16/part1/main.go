package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
)

type Point struct {
	row int
	col int
	dxn int
}

type State struct {
	row   int
	col   int
	dxn   int
	score int
}

type PriorityQueue []*State

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].score < pq[j].score
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x any) {
	*pq = append(*pq, x.(*State))
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

const VISITED rune = 'X'
const WALL rune = '#'
const EMPTY rune = '.'
const EAST = 0
const NORTH = 1
const WEST = 2
const SOUTH = 3

var DXNS = [4][3]int{
	{0, 1, EAST},
	{-1, 0, NORTH},
	{0, -1, WEST},
	{1, 0, SOUTH},
}

func main() {
	file, err := os.Open("../input.in")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	arr := make([][]rune, 0)
	visited := make(map[Point]bool)

	startPoint := Point{0, 0, 0}
	endPoint := Point{0, 0, 0}

	for scanner.Scan() {
		line := scanner.Text()
		row := make([]rune, 0)
		for i, ch := range line {
			if ch == 'S' {
				startPoint.row = len(arr)
				startPoint.col = i
				row = append(row, '>')
			} else if ch == 'E' {
				endPoint.row = len(arr)
				endPoint.col = i
				row = append(row, ch)
			} else {
				row = append(row, ch)
			}
		}
		arr = append(arr, row)
	}
	fmt.Printf("Start: (%d,%d)\nEnd: (%d,%d)\n", startPoint.row, startPoint.col, endPoint.row, endPoint.col)

	// create a priority queue with state
	finalScore := 0
	pq := &PriorityQueue{}
	heap.Init(pq)
	heap.Push(pq, &State{startPoint.row, startPoint.col, EAST, 0})
	for pq.Len() > 0 {
		state := heap.Pop(pq).(*State)
		if state.row == endPoint.row && state.col == endPoint.col {
			finalScore = state.score
			break
		}
		if arr[state.row][state.col] == WALL || visited[Point{state.row, state.col, state.dxn}] {
			// TODO: this might fail if facing-direction matters
			continue
		}
		visited[Point{state.row, state.col, state.dxn}] = true
		// we can explore this state.

		fwd_deltas := DXNS[state.dxn]
		fwd_r := state.row + fwd_deltas[0]
		fwd_c := state.col + fwd_deltas[1]
		left_deltas := DXNS[(state.dxn+1)%4]
		left_r := state.row + left_deltas[0]
		left_c := state.col + left_deltas[1]
		right_deltas := DXNS[(state.dxn-1+4)%4]
		right_r := state.row + right_deltas[0]
		right_c := state.col + right_deltas[1]
		heap.Push(pq, &State{fwd_r, fwd_c, fwd_deltas[2], state.score + 1})
		heap.Push(pq, &State{left_r, left_c, left_deltas[2], state.score + 1001})
		heap.Push(pq, &State{right_r, right_c, right_deltas[2], state.score + 1001})
	}
	fmt.Printf("Score: %d\n", finalScore)

}
