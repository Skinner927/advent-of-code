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

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}

func main() {
	fmt.Println("Hello, World!")

	file, err := os.Open("puzzle1.txt")
	check(err)
	defer func(file *os.File) {
		_ = file.Close()
	}(file)

	var mostCalories, currentCalories int64

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
			if currentCalories > mostCalories {
				mostCalories = currentCalories
			}
			currentCalories = 0
		} else {
			val, err := strconv.ParseInt(line, 10, 64)
			if err != nil {
				log.Fatalf("Failed to parse string '%v'. E: %v\n", line, err)
			}
			currentCalories += val
		}
	}

	fmt.Printf("\n\nWinner: %d\n", mostCalories)
}
