package main

import (
	"bufio"
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
			area, perimeter := search(garden, rows, cols, i, j, char, visited)
			price += area * perimeter
			// fmt.Printf("%c at (%d,%d) is %d, %d\n", char, i, j, area, perimeter)
		}
	}
	for _, row := range garden {
		println(string(row))
	}
	println("Price:", price)

}

var dirs = [4][2]int{
	{1, 0},
	{0, 1},
	{-1, 0},
	{0, -1},
}

func isValid(rows int, cols int, curr_row int, curr_col int) bool {
	return !(curr_row < 0 || curr_row >= rows || curr_col < 0 || curr_col >= cols)
}

// if a cell has 1 neighbour, it has 3 sides
// if a cell has 2 neighbours, it has 2 sides
// if a cell has 3 neighbours, it has 1 side
// if a cell has 4 neighbours, it has 0 sides
func search(garden [][]rune, rows int, cols int, curr_row int, curr_col int, region_char rune, visited [][]bool) (int, int) {
	area, perimeter := 0, 0
	neighbours := 0
	visited[curr_row][curr_col] = true
	for _, dir := range dirs {
		new_r := dir[0] + curr_row
		new_c := dir[1] + curr_col
		if isValid(rows, cols, new_r, new_c) {
			if garden[new_r][new_c] == region_char {
				neighbours += 1
				if !visited[new_r][new_c] {
					delta_area, delta_peri := search(garden, rows, cols, new_r, new_c, region_char, visited)
					area += delta_area
					perimeter += delta_peri
				}
			}
		}
	}
	// fmt.Printf("(%d,%d) has %d neighbours\n", curr_row, curr_col, neighbours)
	perimeter += 4 - neighbours
	if perimeter < 0 {
		panic("peri < 0")
	}
	area += 1
	// garden[curr_row][curr_col] = rune(strconv.Itoa(neighbours)[0])
	return area, perimeter
}
