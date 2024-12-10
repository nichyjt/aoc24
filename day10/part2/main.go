package main

import (
	"bufio"
	"os"
)

func main() {
	file, _ := os.Open("../input.in")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	mountain_map := make([][]int, 0)
	var trailhead_map [][2]int
	rows := 0

	for scanner.Scan() {
		line := scanner.Text()
		row := make([]int, len(line))
		for i, char := range line {
			val := int(char - '0')
			row[i] = val
			if val == 0 {
				trailhead_map = append(trailhead_map, [2]int{rows, i})
			}
		}
		mountain_map = append(mountain_map, row)
		rows++
	}

	rows = len(mountain_map)
	cols := len(mountain_map[0])
	// perform a trail exploration search
	result := 0
	for _, trail := range trailhead_map {
		point_res := search(mountain_map, rows, cols, trail[0], trail[1])
		println("", trail[0], trail[1], point_res)
		result += point_res
	}
	println("Result", result)
}

// should not contain a cycle due to monotonicity
var dirs = [4][2]int{
	{1, 0},
	{0, 1},
	{-1, 0},
	{0, -1},
}

func search(mountain_map [][]int, rows int, cols int, curr_r int, curr_c int) int {
	if mountain_map[curr_r][curr_c] == 9 {
		return 1
	}
	ans := 0
	for _, dir := range dirs {
		new_r := curr_r + dir[0]
		new_c := curr_c + dir[1]
		if new_r < 0 || new_r >= rows || new_c < 0 || new_c >= cols {
			continue
		}
		if mountain_map[new_r][new_c] == mountain_map[curr_r][curr_c]+1 {
			ans += search(mountain_map, rows, cols, new_r, new_c)
		}
	}
	return ans
}
