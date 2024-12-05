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
	page_before_map := make(map[int][]int)
	// until we reach an empty line
	for scanner.Scan() {
		in := scanner.Text()
		if len(in) == 0 {
			break
		}
		strs := strings.Split(in, "|")
		curr_page, _ := strconv.Atoi(strs[0])
		appears_before, _ := strconv.Atoi(strs[1])
		// println(strs[0], strs[1])
		page_before_map[curr_page] = append(page_before_map[curr_page], appears_before)
	}
	result := 0
	for scanner.Scan() {
		str_vals := strings.Split(scanner.Text(), ",")
		var int_vals []int
		for _, s := range str_vals {
			v, _ := strconv.Atoi(s)
			int_vals = append(int_vals, v)
		}
		// Check the rules
		is_correct := true
		seen := make(map[int]bool)
		for _, val := range int_vals {
			if !is_correct {
				break
			}
			// check the nums' before rules
			if should_not_see_list, ok := page_before_map[val]; ok {
				for _, num := range should_not_see_list {
					if seen[num] {
						is_correct = false
						break
					}
				}
			}
			seen[val] = true
		}
		// add the middle result if pass condition
		if is_correct {
			result += int_vals[len(int_vals)/2]
		}
	}
	println("Result:", result)
}
