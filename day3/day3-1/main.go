package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.in")
	defer file.Close()
	if err != nil {
		log.Fatal("Error opening input file:", err)
	}

	scanner := bufio.NewScanner(file)
	pattern := `mul\(\d+,\d+\)`
	re := regexp.MustCompile(pattern)
	result := 0

	for scanner.Scan() {
		line := scanner.Text()
		matches := re.FindAllString(line, -1)
		for _, match := range matches {
			match = match[4 : len(match)-1]
			parts := strings.Split(match, ",")
			left, _ := strconv.Atoi(parts[0])
			right, _ := strconv.Atoi(parts[1])
			result += left * right
		}
	}
	fmt.Printf("Result: %d\n", result)
}
