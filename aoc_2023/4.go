package main

import (
	"aoc/2023/utils"
	"fmt"
	"slices"
	"strings"
)

type CardMultiplication struct {
	cart   []string
	factor int
}

func main() {
	input := strings.Split(utils.ReadFile("2023/input/4.1.txt"), "\n")
	sum := 0
	// --- Part One ---
	//for _, line := range input {
	//	power := 0
	//	winningNumbers := map[string]bool{}
	//	deliminator := slices.Index(strings.Fields(line), "|")
	//	for _, number := range strings.Fields(line)[2:deliminator] {
	//		winningNumbers[number] = true
	//	}
	//	for _, number := range strings.Fields(line)[deliminator:] {
	//		if winningNumbers[number] {
	//			power++
	//		}
	//	}
	//
	//	if power == 0 {
	//		continue
	//	}
	//
	//	sum = sum + 1<<(power-1)
	//}
	// --- Part two ---

	transformedInput := []CardMultiplication{}

	for _, line := range input {
		transformedInput = append(transformedInput, CardMultiplication{
			cart:   strings.Fields(line),
			factor: 1,
		})
	}

	for i, line := range transformedInput {
		power := 0
		winningNumbers := map[string]bool{}
		deliminator := slices.Index(line.cart, "|")
		for _, number := range line.cart[2:deliminator] {
			winningNumbers[number] = true
		}

		for _, number := range line.cart[deliminator:] {
			if winningNumbers[number] {
				power++
			}
		}

		for j := 1; j <= power && j < len(transformedInput); j++ {
			transformedInput[i+j].factor += transformedInput[i].factor
		}

	}

	for _, line := range transformedInput {
		sum += line.factor
	}

	fmt.Println(sum)
}
