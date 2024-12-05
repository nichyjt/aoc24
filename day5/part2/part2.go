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
		page_before_map[curr_page] = append(page_before_map[curr_page], appears_before)
	}

	result := 0

	var incorrect_pages [][]int
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
		if !is_correct {
			incorrect_pages = append(incorrect_pages, int_vals)
		}
	}

	// Could use a toposort, but seems unnecessarily complicated
	for _, incorrect_page := range incorrect_pages {
		for {
			restart := false
			seen := make(map[int]int)
			for i, val := range incorrect_page {
				seen[val] = i
				// check the nums' before rules
				if should_not_see_list, ok := page_before_map[val]; ok {
					for _, num := range should_not_see_list {
						if j, exists := seen[num]; exists {
							incorrect_page[i], incorrect_page[j] = incorrect_page[j], incorrect_page[i]
							restart = true
							break
						}
					}
				}
				if restart {
					break
				}
			}
			if !restart {
				// incorrect_page solved!
				break
			}
		}
		// add the result of the sorted page
		result += incorrect_page[len(incorrect_page)/2]
	}
	println("Result:", result)
}
