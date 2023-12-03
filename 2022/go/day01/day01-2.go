package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

func fillCalories(mostCalories *[3]int64, currentCalories int64) {
	// First fill zeroes
	if 0 == mostCalories[2] {
		for i, v := range mostCalories {
			if 0 == v {
				mostCalories[i] = currentCalories
				return
			}
		}
	}

	for i, v := range mostCalories {
		if currentCalories > v {
			mostCalories[i] = currentCalories
			return
		}
	}
}

func main() {
	fmt.Println("Hello, World!")

	file, err := os.Open("puzzle1.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer func(file *os.File) {
		_ = file.Close()
	}(file)

	mostCalories := [3]int64{}
	var currentCalories int64

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
			fmt.Printf("Elf done cals: %d  most: %d\n", currentCalories, mostCalories)
			fillCalories(&mostCalories, currentCalories)
			currentCalories = 0
		} else {
			val, err := strconv.ParseInt(line, 10, 64)
			if err != nil {
				log.Fatalf("Failed to parse string '%v'. E: %v\n", line, err)
			}
			currentCalories += val
		}
	}

	var total int64
	for _, v := range mostCalories {
		total += v
	}
	fmt.Printf("\n\nWinner: %d\n", total)
}
