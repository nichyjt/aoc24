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

type Coord struct {
	row int
	col int
}

type State struct {
	row   int
	col   int
	dxn   int
	score int
	path  []Coord
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

const (
	VISITED rune = 'X'
	WALL    rune = '#'
	EMPTY   rune = '.'
	EAST         = 0
	NORTH        = 1
	WEST         = 2
	SOUTH        = 3
)

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
	visited := make(map[Point]int)

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

	finalScore := -1
	var bestPaths [][]Coord

	pq := &PriorityQueue{}
	heap.Init(pq)

	initialPath := []Coord{{startPoint.row, startPoint.col}}
	heap.Push(pq, &State{startPoint.row, startPoint.col, EAST, 0, initialPath})

	for pq.Len() > 0 {
		state := heap.Pop(pq).(*State)

		if state.row == endPoint.row && state.col == endPoint.col {
			if finalScore == -1 || state.score == finalScore {
				finalScore = state.score
				bestPaths = append(bestPaths, state.path)
			}
		}

		if arr[state.row][state.col] == WALL {
			continue
		}

		currentPoint := Point{state.row, state.col, state.dxn}
		if prevScore, seen := visited[currentPoint]; seen && prevScore < state.score {
			continue
		}
		visited[currentPoint] = state.score

		// Forward
		fwdDeltas := DXNS[state.dxn]
		fwdR := state.row + fwdDeltas[0]
		fwdC := state.col + fwdDeltas[1]
		fwdPath := append([]Coord{}, state.path...)
		fwdPath = append(fwdPath, Coord{fwdR, fwdC})
		heap.Push(pq, &State{fwdR, fwdC, fwdDeltas[2], state.score + 1, fwdPath})

		// Left
		leftDeltas := DXNS[(state.dxn+1)%4]
		leftR := state.row + leftDeltas[0]
		leftC := state.col + leftDeltas[1]
		leftPath := append([]Coord{}, state.path...)
		leftPath = append(leftPath, Coord{leftR, leftC})
		heap.Push(pq, &State{leftR, leftC, leftDeltas[2], state.score + 1001, leftPath})

		// Right
		rightDeltas := DXNS[(state.dxn-1+4)%4]
		rightR := state.row + rightDeltas[0]
		rightC := state.col + rightDeltas[1]
		rightPath := append([]Coord{}, state.path...)
		rightPath = append(rightPath, Coord{rightR, rightC})
		heap.Push(pq, &State{rightR, rightC, rightDeltas[2], state.score + 1001, rightPath})
	}

	allTiles := []Coord{}
	for _, path := range bestPaths {
		allTiles = append(allTiles, path...)
	}
	uniqueTiles := countUniqueTiles(allTiles)
	fmt.Printf("Score: %d\nTiles = %d\n", finalScore, uniqueTiles)
	// debug print
	// Print one of the paths on the map
	// if len(bestPaths) > 0 {
	// 	for _, path := range bestPaths {
	// 		for _, coord := range path {
	// 			arr[coord.row][coord.col] = 'O'
	// 		}
	// 	}
	// }
	// for _, row := range arr {
	// 	for _, v := range row {
	// 		fmt.Printf("%c", v)
	// 	}
	// 	println()
	// }
}

func countUniqueTiles(path []Coord) int {
	tileSet := make(map[Coord]struct{})
	for _, coord := range path {
		tileSet[coord] = struct{}{}
	}
	return len(tileSet)
}
