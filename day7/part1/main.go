package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../debug.in")
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
		vals := make([]int, len(val_strs))
		for i, val_str := range val_strs {
			val, _ := strconv.Atoi(val_str)
			vals[i] = val
		}
		// now, we perform a backtracking algorithm
		if solve(target, vals, 1, vals[0]) {
			result += target
		}
	}
	println("Result:", result)
}

// WARN: start with curr_idx = 1
func solve(target int, vals []int, curr_idx int, curr_val int) bool {
	if curr_idx >= len(vals) {
		return target == curr_val
	}
	// try +
	curr_val += vals[curr_idx]
	if solve(target, vals, curr_idx+1, curr_val) {
		return true
	}
	curr_val -= vals[curr_idx]
	// try *
	curr_val *= vals[curr_idx]
	if solve(target, vals, curr_idx+1, curr_val) {
		return true
	}
	return false
}
