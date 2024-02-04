package main

import (
	"aoc/2023/utils"
	"fmt"
	"strings"
)

type TreeNode struct {
	left  string
	right string
}

type Graph map[string]TreeNode

func ParseInput(input []string) *Graph {
	graph := make(Graph)
	for _, node := range input {
		parts := strings.Fields(node)
		graph[parts[0]] = TreeNode{parts[2][1:4], parts[3][:3]}
	}

	return &graph
}

func main() {
	input := strings.Split(utils.ReadFile("input/8.1.txt"), "\n")
	moveLoop := input[0]
	graph := ParseInput(input[2:])
	start := "AAA"
	end := "ZZZ"
	curNode := start
	moveCount := 0
	fmt.Println(moveLoop, start, end)
	for curNode != end && curNode != "" {
		fmt.Println(curNode)
		if rune(moveLoop[moveCount%len(moveLoop)]) == 'L' {
			curNode = (*graph)[curNode].left
		} else {
			curNode = (*graph)[curNode].right
		}
		moveCount++
	}

	fmt.Println(moveCount)

}
