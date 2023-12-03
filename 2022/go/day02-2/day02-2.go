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
	/*
	   Now our hand means that we need to:
	     Rock == lose
	     Paper == draw
	     Scissors == win
	*/
	var ourChoice Hand
	ourAction := WIN
	switch hand {
	case Rock:
		ourAction = LOSS
	case Paper:
		ourAction = DRAW
	case Scissors:
		ourAction = WIN
	}

	switch theirs {
	case Rock:
		switch ourAction {
		case WIN:
			ourChoice = Paper
		case DRAW:
			ourChoice = Rock
		case LOSS:
			ourChoice = Scissors
		}
	case Paper:
		switch ourAction {
		case WIN:
			ourChoice = Scissors
		case DRAW:
			ourChoice = Paper
		case LOSS:
			ourChoice = Rock
		}
	case Scissors:
		switch ourAction {
		case WIN:
			ourChoice = Rock
		case DRAW:
			ourChoice = Scissors
		case LOSS:
			ourChoice = Paper
		}
	}

	return ourChoice.Score() + ourAction
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
