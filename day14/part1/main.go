package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../input.in")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	const ROWS int = 103 //103
	const COLS int = 101 //101
	const ROW_EXCLUDE = (ROWS+1)/2 - 1
	const COL_EXCLUDE = (COLS+1)/2 - 1
	fmt.Printf("%d, %d\n", ROW_EXCLUDE, COL_EXCLUDE)
	const SIM_TIME = 100

	scanner := bufio.NewScanner(file)

	score := 0
	q1Count := 0
	q2Count := 0
	q3Count := 0
	q4Count := 0

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		positionStr := parts[0]
		velocityStr := parts[1]

		positionParts := strings.Split(positionStr[2:], ",")
		pc, _ := strconv.Atoi(positionParts[0]) // x
		pr, _ := strconv.Atoi(positionParts[1]) // y

		velocityParts := strings.Split(velocityStr[2:], ",")
		vc, _ := strconv.Atoi(velocityParts[0]) // vx
		vr, _ := strconv.Atoi(velocityParts[1]) // vy

		// Perform the simulation of 100 ticks with wrapping
		pr = pr + SIM_TIME*vr
		pc = pc + SIM_TIME*vc
		// torus wrap
		pr = (pr%ROWS + ROWS) % ROWS
		pc = (pc%COLS + COLS) % COLS
		// fmt.Printf("FINAL: (%d,%d)\n", pr, pc)

		// exclude those that lie perfectly on ROW_EXCLUDE, COL_EXCLUDE
		if pr < (ROW_EXCLUDE) {
			if pc < (COL_EXCLUDE) {
				q1Count++
			} else if pc > (COL_EXCLUDE) {
				q2Count++
			}
		} else if pr > (ROW_EXCLUDE) {
			if pc < (COL_EXCLUDE) {
				q3Count++
			} else if pc > (COL_EXCLUDE) {
				q4Count++
			}
		}
	}

	// Calculate the final score
	score += q1Count * q2Count * q3Count * q4Count
	fmt.Printf("Score: %d*%d*%d*%d=%d\n", q1Count, q2Count, q3Count, q4Count, score)
}
