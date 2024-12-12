package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, _ := os.Open("../input.in")
	defer file.Close()
	scanner := bufio.NewScanner(file)
	var garden [][]rune
	for scanner.Scan() {
		line := []rune(scanner.Text())
		garden = append(garden, line)
	}
	rows := len(garden)
	cols := len(garden[0])

	visited := make([][]bool, rows)
	for i := range visited {
		visited[i] = make([]bool, cols)
	}

	price := 0
	for i, row := range garden {
		for j := range row {
			if visited[i][j] {
				continue
			}
			char := garden[i][j]
			area, sides := search(garden, rows, cols, i, j, char, visited)
			fmt.Printf("%c = %d, %d\n", char, area, sides)
			price += area * sides
		}
	}

	println("Price:", price)

}

var dirs = [4][2]int{
	{-1, 0}, //up
	{1, 0},  //down
	{0, -1}, //left
	{0, 1},  //right
}

var new_dirs = [8][2]int{
	{-1, 0},  //up
	{1, 0},   //down
	{0, -1},  //left
	{0, 1},   //right
	{-1, 1},  //upright
	{-1, -1}, //upleft
	{1, 1},   //downright
	{1, -1},  //downleft
}

const UP int = 0
const DOWN int = 1
const LEFT int = 2
const RIGHT int = 3
const UP_RIGHT int = 4
const UP_LEFT int = 5
const DOWN_RIGHT int = 6
const DOWN_LEFT int = 7

func isValid(rows int, cols int, curr_row int, curr_col int) bool {
	return !(curr_row < 0 || curr_row >= rows || curr_col < 0 || curr_col >= cols)
}

func isOccupied(garden [][]rune, rows int, cols int, curr_row int, curr_col int, region_char rune, dxn int) bool {
	dir := new_dirs[dxn]
	new_r := dir[0] + curr_row
	new_c := dir[1] + curr_col
	return isValid(rows, cols, new_r, new_c) && garden[new_r][new_c] == region_char
}

func countCorners(garden [][]rune, rows int, cols int, curr_row int, curr_col int, region_char rune) int {
	n := [8]bool{false}
	m := 0
	for i, dir := range dirs {
		new_r := dir[0] + curr_row
		new_c := dir[1] + curr_col
		if isValid(rows, cols, new_r, new_c) && garden[new_r][new_c] == region_char {
			n[i] = true
			m += 1
		}
	}
	// process occupied
	if m == 0 {
		// No neighbors -> isolated cell
		return 4
	}
	if m == 1 {
		// Single neighbor, either straight or diagonal
		if n[LEFT] || n[RIGHT] || n[UP] || n[DOWN] {
			return 2
		}
		return 0 // Single diagonal neighbor doesn't add corners
	}
	// m >= 2
	// Two neighbors
	if m == 2 {
		if (n[UP] && n[DOWN]) || (n[LEFT] && n[RIGHT]) {
			// Opposing neighbors -> no corners
			return 0
		}
	}
	corners := 0
	if n[UP] && n[LEFT] {
		if !isOccupied(garden, rows, cols, curr_row, curr_col, region_char, UP_LEFT) {
			corners += 1
		}
	}
	if n[UP] && n[RIGHT] {
		if !isOccupied(garden, rows, cols, curr_row, curr_col, region_char, UP_RIGHT) {
			corners += 1
		}
	}
	if n[DOWN] && n[LEFT] {
		if !isOccupied(garden, rows, cols, curr_row, curr_col, region_char, DOWN_LEFT) {
			corners += 1
		}
	}
	if n[DOWN] && n[RIGHT] {
		if !isOccupied(garden, rows, cols, curr_row, curr_col, region_char, DOWN_RIGHT) {
			corners += 1
		}
	}
	//
	if !n[UP] && !n[LEFT] {
		corners += 1
	}
	if !n[UP] && !n[RIGHT] {
		corners += 1
	}
	if !n[DOWN] && !n[LEFT] {
		corners += 1
	}
	if !n[DOWN] && !n[RIGHT] {
		corners += 1
	}
	return corners
}

// the trick is to count corners; |corners| == |sides|
func search(garden [][]rune, rows int, cols int, curr_row int, curr_col int, region_char rune, visited [][]bool) (int, int) {
	area, sides := 0, 0
	n := []bool{false, false, false, false} //neighbours
	neighbours := n                         //alias
	num_neighbours := 0
	visited[curr_row][curr_col] = true
	for i, dir := range dirs {
		new_r := dir[0] + curr_row
		new_c := dir[1] + curr_col
		if isValid(rows, cols, new_r, new_c) {
			if garden[new_r][new_c] == region_char {
				neighbours[i] = true
				num_neighbours += 1
				if !visited[new_r][new_c] {
					delta_area, delta_sides := search(garden, rows, cols, new_r, new_c, region_char, visited)
					area += delta_area
					sides += delta_sides
				}
			}
		}
	}
	this_side := countCorners(garden, rows, cols, curr_row, curr_col, region_char)
	fmt.Printf("%c (%d,%d) = %d\n", region_char, curr_row, curr_col, this_side)
	sides += this_side
	area += 1
	// garden[curr_row][curr_col] = rune(strconv.Itoa(neighbours)[0])
	return area, sides
}
