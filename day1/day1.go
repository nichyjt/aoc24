package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.in")
	defer file.Close()
	if err != nil {
		log.Fatal("err opening input file")
	}
	scanner := bufio.NewScanner(file)
	// Populate the inputs
	var leftArr []int
	var rightArr []int
	for scanner.Scan() {
		line := scanner.Text()
		strs := strings.Split(line, "   ")
		left, _ := strconv.Atoi(strs[0])
		right, _ := strconv.Atoi(strs[1])

		leftArr = append(leftArr, left)
		rightArr = append(rightArr, right)
	}
	// Sort them before processing
	sort.Ints(leftArr)
	sort.Ints(rightArr)

	var result int
	for i := 0; i < len(leftArr); i++ {
		// go has no int abs for some godforsaken reason
		tmp := leftArr[i] - rightArr[i]
		result += max(tmp, tmp*-1)
	}
	fmt.Printf("The answer is: %d\n", result)
}
