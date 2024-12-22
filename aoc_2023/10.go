package main

import (
	"aoc/2023/utils"
	"fmt"
	"image"
	"strings"
)

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
var directions = map[string]struct {
	i int
	j int
}{
	"|": {1, 0},
	"-": {0, 1},
	"L": {1, 1},
	"J": {1, 0},
	"7": {-1, -1},
	"F": {-1, 1},
	".": {0, 0},
	"S": {0, 0},
}

func findStart(arg *[][]string) image.Point {
	for i, col := range *arg {
		for j, row := range col {
			if row == "S" {
				return image.Point{i, j}
			}
		}
	}
	return image.Point{0, 0}
}

func findFirstPipe(arg *[][]string, point *image.Point) image.Point {
	if (*arg)[point.X][point.Y+1] != "." {
		return image.Point{point.X, point.Y + 1}
	}
	if (*arg)[point.X][point.Y-1] != "." {
		return image.Point{point.X, point.Y - 1}
	}
	if (*arg)[point.X+1][point.Y] != "." {
		return image.Point{point.X + 1, point.Y}
	}
	if (*arg)[point.X-1][point.Y] != "." {
		return image.Point{point.X - 1, point.Y}
	}
	return image.Point{0, 0}
}

func traverseAndCount(arg *[][]string) int {
	current := findStart(arg)
	next := findFirstPipe(arg, &current)
	var count int

	for {
		temp := image.Point{
			current.X + directions[(*arg)[next.X][next.Y]].i,
			current.Y + directions[(*arg)[next.X][next.Y]].j,
		}
		fmt.Println(current, next, (*arg)[temp.X][temp.Y])
		count++
		if (*arg)[temp.X][temp.Y] == "." {
			return count
		}

		current = next
		next = temp
	}
}

func main() {
	input := strings.Split(utils.ReadFile("input/10.txt"), "\n")
	grid := make([][]string, len(input)+2)
	grid[0] = make([]string, len(input[0])+2)
	for i, line := range input {
		grid[i+1] = strings.Split("."+line+".", "")
	}
	grid[len(input)+1] = make([]string, len(input[0])+2)
	fmt.Println(grid)
	println(traverseAndCount(&grid))
}
