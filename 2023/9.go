package main

import (
	"aoc/2023/utils"
	"fmt"
	"strconv"
	"strings"
)

func isDifferenceEqualTo(arr *[]int, equalTo int) bool {
	var difference int
	for i := 1; i < len(*arr); i++ {
		difference = (*arr)[i] - (*arr)[i-1]
		if difference != equalTo {
			return false
		}
	}
	return true
}

func calculateDifferenceArray(array *[]int) *[]int {
	var newDifference []int
	for i := 1; i < len(*array); i++ {
		newDifference = append(newDifference, (*array)[i]-(*array)[i-1])
	}

	return &newDifference
}

func findExtrapolation(currentDifference *[]int, lastValue int) int {
	if len(*currentDifference) == 1 || isDifferenceEqualTo(currentDifference, 0) {
		return (*currentDifference)[0]
	}

	newDifference := calculateDifferenceArray(currentDifference)
	//fmt.Println(newDifference, currentDifference)
	extrapolatedValue := findExtrapolation(newDifference, (*newDifference)[0])
	//fmt.Println(extrapolatedValue, lastValue)
	return lastValue - extrapolatedValue
}

func parseArray(arr []string) (*[]int, error) {
	var newArray = make([]int, 0)
	for _, value := range arr {
		parsedValue, err := strconv.ParseInt(value, 10, 64)
		if err != nil {
			return nil, err
		}
		newArray = append(newArray, int(parsedValue))
	}

	return &newArray, nil
}

func main() {
	input := strings.Split(utils.ReadFile("input/9.1.txt"), "\n")

	sum := 0
	for _, line := range input {
		parsedLine, err := parseArray(strings.Fields(line))
		if err != nil {
			panic(err)
		}

		sum += findExtrapolation(parsedLine, (*parsedLine)[0])
		fmt.Println(fmt.Sprintf("After line: %v, sum is %d", line, sum))
	}

	fmt.Println(sum)
}
