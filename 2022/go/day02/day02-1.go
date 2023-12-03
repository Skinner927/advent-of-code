package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}

type Hand int

const (
	Rock Hand = iota
	Paper
	Scissors
)

func (hand Hand) String() string {
	switch hand {
	case Rock:
		return "Rock"
	case Paper:
		return "Paper"
	case Scissors:
		return "Scissors"
	}
	return "UNKNOWN"
}

func ParseHand(msg string) Hand {
	switch msg {
	case "A":
		return Rock
	case "B":
		return Paper
	case "C":
		return Scissors
	case "X":
		return Rock
	case "Y":
		return Paper
	case "Z":
		return Scissors
	}
	panic(msg)
}

func (hand Hand) Score() int {
	switch hand {
	case Rock:
		return 1
	case Paper:
		return 2
	case Scissors:
		return 3
	}
	return -10000
}

const (
	WIN  int = 6
	DRAW     = 3
	LOSS     = 0
)

func (hand Hand) Play(theirs Hand) int {
	switch theirs {
	case Rock:
		switch hand {
		case Rock:
			return hand.Score() + DRAW
		case Paper:
			return hand.Score() + WIN
		case Scissors:
			return hand.Score() + LOSS
		}
	case Paper:
		switch hand {
		case Rock:
			return hand.Score() + LOSS
		case Paper:
			return hand.Score() + DRAW
		case Scissors:
			return hand.Score() + WIN
		}
	case Scissors:
		switch hand {
		case Rock:
			return hand.Score() + WIN
		case Paper:
			return hand.Score() + LOSS
		case Scissors:
			return hand.Score() + DRAW
		}
	}

	return -2000
}

func main() {
	file, err := os.Open("puzzle1.txt")
	check(err)
	defer func(file *os.File) {
		_ = file.Close()
	}(file)

	ourScore := 0

	reader := bufio.NewReader(file)
	for {
		lineBytes, _, err := reader.ReadLine()
		if err != nil {
			if err != io.EOF {
				log.Fatal("Failed to read line: ", err)
			}
			break
		}

		line := strings.TrimSpace(string(lineBytes[:]))
		if line == "" {
			continue
		}

		parts := strings.Split(line, " ")
		fmt.Printf("Got line: %s  parts: %v\n", line, parts)

		ourScore += ParseHand(parts[1]).Play(ParseHand(parts[0]))
	}

	fmt.Printf("\n\nourScore: %d\n", ourScore)
}
