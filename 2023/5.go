package main

import (
	"aoc/2023/utils"
	"fmt"
	"slices"
	"strconv"
	"strings"
	"syscall"
)

type Interval struct {
	IntervalStart int
	Interval      int
}

type MappingInterval struct {
	IntervalSrc   int
	IntervalDes   int
	IntervalRange int
}

func mapValue(input []string, value int) int {
	desMapping := value
	for i := 0; i < len(input) && len(strings.TrimSpace(input[i])) != 0; i++ {
		values := strings.Split(input[i], " ")
		des, _ := strconv.ParseInt(values[0], 10, 64)
		src, _ := strconv.ParseInt(values[1], 10, 64)
		rng, _ := strconv.ParseInt(values[2], 10, 64)
		if int(src) <= value && value <= int(src)+int(rng) {
			desMapping = int(des) + (value - int(src))
		}
	}

	return desMapping
}

func checkIfMappingIntersectsWithInterval(interval Interval, mappingInterval MappingInterval) bool {
	return interval.IntervalStart <= mappingInterval.IntervalSrc && mappingInterval.IntervalSrc <= interval.IntervalStart+interval.Interval ||
		interval.IntervalStart <= mappingInterval.IntervalSrc+mappingInterval.IntervalRange && mappingInterval.IntervalSrc+mappingInterval.IntervalRange <= interval.IntervalStart+interval.Interval
}

func mapRange(interval Interval, mappingInterval []MappingInterval) []Interval {
	var resultingInterval []Interval

}

func getSeedsIntervals(seedsWithInterval *[]string) (*[]Interval, error) {
	var seedsIntervals []Interval
	for i := 0; i < len(*seedsWithInterval); i += 2 {
		intervalStart, err := strconv.ParseInt((*seedsWithInterval)[i], 10, 64)
		if err != nil {
			return nil, err
		}
		interval, err := strconv.ParseInt((*seedsWithInterval)[i+1], 10, 64)
		if err != nil {
			return nil, err
		}
		seedsIntervals = append(seedsIntervals, Interval{int(intervalStart), int(interval)})
	}

	return &seedsIntervals, nil
}

func getMappingRange(rangeValues []string) ([]MappingInterval, error) {
	var mappingRange []MappingInterval
	for i := 0; i < len(rangeValues); i++ {
		rangeValuesSplit := strings.Split(rangeValues[i], " ")
		fmt.Printf("%v\n", rangeValuesSplit)
		if len(strings.TrimSpace(rangeValues[i])) == 0 {
			break
		}
		intervalDes, err := strconv.ParseInt(rangeValuesSplit[0], 10, 64)
		if err != nil {
			return nil, err
		}
		intervalSrc, err := strconv.ParseInt(rangeValuesSplit[1], 10, 64)
		if err != nil {
			return nil, err
		}
		intervalRange, err := strconv.ParseInt(rangeValuesSplit[2], 10, 64)
		if err != nil {
			return nil, err
		}
		mappingRange = append(mappingRange, MappingInterval{int(intervalSrc), int(intervalDes), int(intervalRange)})
	}

	return mappingRange, nil
}

func main() {
	input := strings.Split(utils.ReadFile("2023/input/5.txt"), "\n")

	// ---- Part one ----
	//for _, v := range seedsIntervals {
	//	parsedSeed, _ := strconv.ParseInt(v, 10, 64)
	//	location :=
	//		mapValue(input[slices.Index(input, "humidity-to-location map:")+1:],
	//			mapValue(input[slices.Index(input, "temperature-to-humidity map:")+1:],
	//				mapValue(input[slices.Index(input, "light-to-temperature map:")+1:],
	//					mapValue(input[slices.Index(input, "water-to-light map:")+1:],
	//						mapValue(input[slices.Index(input, "fertilizer-to-water map:")+1:],
	//							mapValue(input[slices.Index(input, "soil-to-fertilizer map:")+1:],
	//								mapValue(input[slices.Index(input, "IntervalStart-to-soil map:")+1:], int(parsedSeed))))))))
	//	answer = int(math.Min(float64(location), float64(answer)))
	//}

	seedsWithInterval := strings.Split(input[0][7:], " ")
	fmt.Println(seedsWithInterval)
	seeds, err := getSeedsIntervals(&seedsWithInterval)
	if err != nil {
		fmt.Println(err)
		syscall.Exit(1)
	}

	seedsToSoilMapping, err := getMappingRange(input[slices.Index(input, "seed-to-soil map:")+1:])
	if err != nil {
		fmt.Println(err)
		syscall.Exit(1)
	}

}
