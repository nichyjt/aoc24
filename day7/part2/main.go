package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../input.in")
	defer file.Close()
	if err != nil {
		log.Fatal("Error opening input file:", err)
	}

	scanner := bufio.NewScanner(file)
	// until we reach an empty line
	result := 0
	for scanner.Scan() {
		text := scanner.Text()
		texts := strings.Split(text, ":")
		target, _ := strconv.Atoi(texts[0])
		val_strs := strings.Split(texts[1], " ")
		// now, we perform a backtracking algorithm
		initial_val, _ := strconv.Atoi(val_strs[0])
		if solve(target, val_strs, 1, initial_val) {
			result += target
		}
	}
	println("Result:", result)
}

// WARN: start with curr_idx = 1
func solve(target int, vals []string, curr_idx int, curr_val int) bool {
	if curr_idx >= len(vals) {
		return target == curr_val
	}
	// try ||
	curr_str := strconv.Itoa(curr_val)
	new_str := curr_str + vals[curr_idx]
	new_val, _ := strconv.Atoi(new_str)
	if solve(target, vals, curr_idx+1, new_val) {
		return true
	}
	// try +
	curr_new, _ := strconv.Atoi(vals[curr_idx])
	curr_val += curr_new
	if solve(target, vals, curr_idx+1, curr_val) {
		return true
	}
	curr_val -= curr_new
	// try *
	curr_val *= curr_new
	if solve(target, vals, curr_idx+1, curr_val) {
		return true
	}
	return false
}
