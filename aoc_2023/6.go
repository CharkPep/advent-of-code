package main

import (
	"aoc/2023/utils"
	"fmt"
	"strconv"
	"strings"
)

type Record struct {
	limitTime      int
	recordDistance int
}

func parseInput(timeLimits, distanceRecords []string) []Record {
	records := make([]Record, len(timeLimits))
	for i := range records {
		parsedTime, _ := strconv.ParseInt(timeLimits[i], 10, 64)
		records[i].limitTime = int(parsedTime)
		parsedDistance, _ := strconv.ParseInt(distanceRecords[i], 10, 64)
		records[i].recordDistance = int(parsedDistance)
	}
	return records
}

func calculateWaysToBeatRecordTime(record *Record) int {
	ways := 0
	for i := 1; i <= record.limitTime; i++ {
		if (record.limitTime-i)*i > record.recordDistance {
			ways++
		}
	}

	return ways
}

func main() {
	input := strings.Split(utils.ReadFile("2023/input/6.1.txt"), "\n")
	// Part 1
	//timeLimits := strings.Fields(input[0])[1:]
	//distanceRecords := strings.Fields(input[1])[1:]
	//records := parseInput(timeLimits, distanceRecords)
	//sum := 1
	//for _, record := range records {
	//	sum *= calculateWaysToBeatRecordTime(&record)
	//}
	//
	//fmt.Println(sum)
	// Part 2
	timeLimits, _ := strconv.ParseInt(strings.Join(strings.Fields(input[0])[1:], ""), 10, 64)
	distanceRecords, _ := strconv.ParseInt(strings.Join(strings.Fields(input[1])[1:], ""), 10, 64)
	records := []Record{{int(timeLimits), int(distanceRecords)}}
	sum := 1
	for _, record := range records {
		sum *= calculateWaysToBeatRecordTime(&record)
	}

	fmt.Println(sum)
}
