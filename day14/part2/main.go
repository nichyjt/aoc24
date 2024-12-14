package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Bot struct {
	pr int
	pc int
	vr int
	vc int
}

func main() {
	file, err := os.Open("../input.in")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	const ROWS int = 103
	const COLS int = 101
	const ROW_EXCLUDE = (ROWS+1)/2 - 1
	const COL_EXCLUDE = (COLS+1)/2 - 1
	fmt.Printf("%d, %d\n", ROW_EXCLUDE, COL_EXCLUDE)
	const SIM_TIME = 100_000

	scanner := bufio.NewScanner(file)

	bots := []Bot{}
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

		bot := Bot{pr, pc, vr, vc}
		bots = append(bots, bot)
	}

	str := strings.Repeat("=", 80)

	// Perform the simulation of 100 ticks with wrapping
	for i := 0; i < SIM_TIME; i++ {
		easterEggMap := [ROWS][COLS]int{} // Use int for bot count
		tick := i + 1

		// Update bot positions and populate the map
		for _, bot := range bots {
			pr := bot.pr + bot.vr*tick
			pc := bot.pc + bot.vc*tick
			pr = (pr%ROWS + ROWS) % ROWS
			pc = (pc%COLS + COLS) % COLS
			easterEggMap[pr][pc]++
		}

		// Check if any row has more than 8 bots in a row
		found := false
		for _, row := range easterEggMap {
			consecutive := 0
			for _, val := range row {
				if val > 0 {
					consecutive++
					if consecutive >= 8 {
						found = true
						break
					}
				} else {
					consecutive = 0
				}
			}
			if found {
				break
			}
		}

		// If a row with more than 8 bots in a row is found, print the map
		if found {
			fmt.Printf("Iter %d %s\n", tick, str)
			for _, row := range easterEggMap {
				for _, val := range row {
					if val == 0 {
						fmt.Printf(" ")
					} else {
						fmt.Printf("0")
					}
				}
				fmt.Println()
			}
			fmt.Println()
		}
	}
}
