package main

import (
	"aoc/2023/utils"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
var cardValue = map[string]int{
	"A": 1,
	"K": 2,
	"Q": 3,
	"T": 5,
	"9": 6,
	"8": 7,
	"7": 8,
	"6": 9,
	"5": 10,
	"4": 11,
	"3": 12,
	"2": 13,
	"J": 14,
}

type HandType int64

const (
	FiveOfAKind HandType = iota
	FourOfAKind
	FullHouse
	ThreeOfAKind
	TwoPair
	OnePair
	HighCard
)

type Hand struct {
	cards []string
	bid   int
}

func (hand HandType) String() string {
	return [...]string{"FiveOfAKind", "FourOfAKind", "FullHouse", "ThreeOfAKind", "TwoPair", "OnePair", "HighCard"}[hand]
}

func countEntriesOfHand(hand Hand) map[string]int {
	entries := map[string]int{}
	for _, card := range hand.cards {
		entries[card]++
	}
	return entries

}

func getHandType(handType Hand) HandType {
	entries := countEntriesOfHand(handType)
	jokers := entries["J"]
	entries["J"] = 0
	isOnePair := "."
	isThreeOfAKind := "."
	for c, v := range entries {
		if v+jokers == 5 {
			return FiveOfAKind
		}
		if v+jokers == 4 {
			return FourOfAKind
		}
		if v+jokers == 3 || v == 3 {
			isThreeOfAKind = c
		}
		if v+jokers == 2 || v == 2 {
			if entries[isThreeOfAKind]+v+jokers == 5 && isThreeOfAKind != c {
				fmt.Println("full house")
				return FullHouse
			}
			if isOnePair != "." && entries[isOnePair]+v+jokers == 4 {
				return TwoPair
			}
			isOnePair = c
		}
	}

	if isThreeOfAKind != "." && isOnePair != "." && entries[isThreeOfAKind]+entries[isOnePair]+jokers == 5 {
		return FullHouse
	}

	if isThreeOfAKind != "." {
		return ThreeOfAKind
	}
	if isOnePair != "." {
		return OnePair
	}
	return HighCard

}

func comparator(a, b Hand) int {
	handTypeA := getHandType(a)
	handTypeB := getHandType(b)
	fmt.Println(a.cards, handTypeA)
	fmt.Println(b.cards, handTypeB)
	if handTypeA > handTypeB {
		return 1
	}
	if handTypeA < handTypeB {
		return -1
	}

	for i := 0; i < len(a.cards); i++ {
		if cardValue[a.cards[i]] < cardValue[b.cards[i]] {
			fmt.Println(a.cards[i], b.cards[i])
			return -1
		}
		if cardValue[a.cards[i]] > cardValue[b.cards[i]] {
			fmt.Println(a.cards[i], b.cards[i])
			return 1
		}
	}

	return 0
}

func main() {
	input := strings.Split(utils.ReadFile("input/7.1.txt"), "\n")
	//input := []string{"KKKK2 2", "AJBCA 1"}
	var hands []Hand
	for _, line := range input {
		hand := strings.Fields(line)[0]
		bid, _ := strconv.ParseInt(strings.Fields(line)[1], 10, 64)
		hands = append(hands, Hand{strings.Split(hand, ""), int(bid)})
	}

	slices.SortFunc(hands, comparator)
	slices.Reverse(hands)
	fmt.Println(hands)
	sum := 0
	for i, hand := range hands {
		sum += hand.bid * (i + 1)
	}
	fmt.Println(sum)
}
