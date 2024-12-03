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
	file, err := os.Open("../input.in")
	defer file.Close()
	if err != nil {
		log.Fatal("Error opening input file:", err)
	}

	scanner := bufio.NewScanner(file)
	pattern := `mul\(\d+,\d+\)`
	re_mul := regexp.MustCompile(pattern)
	re_do := regexp.MustCompile(`do()`)
	re_dont := regexp.MustCompile(`don't()`)
	result := 0
	input_string := ""
	for scanner.Scan() {
		input_string += scanner.Text()
	}

	is_enabled := true
	end_idx_first := 0
	end_idx_second := 0
	for len(input_string) > 0 {
		if is_enabled {
			end_idxes := re_dont.FindStringIndex(input_string)
			if end_idxes == nil {
				end_idx_first = len(input_string)
				end_idx_second = len(input_string)
			} else {
				end_idx_first = end_idxes[0]
				end_idx_second = end_idxes[1]
			}
			target_text := input_string[:end_idx_first]
			matches := re_mul.FindAllString(target_text, -1)
			for _, match := range matches {
				match = match[4 : len(match)-1]
				parts := strings.Split(match, ",")
				left, _ := strconv.Atoi(parts[0])
				right, _ := strconv.Atoi(parts[1])
				result += left * right
			}
			input_string = input_string[end_idx_second:]
			is_enabled = false
		} else {
			end_idxes := re_do.FindStringIndex(input_string)
			if end_idxes == nil {
				break
			}
			input_string = input_string[end_idxes[0]:]
			is_enabled = true
		}
	}

	fmt.Printf("Result: %d\n", result)
}
