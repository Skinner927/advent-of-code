package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const gFilename = "../day01-1/input.txt"

// const gFilename = "example.txt"

var gNumberList = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
var gAllNumbersReg = regexp.MustCompile(`[1-9]|` + strings.Join(gNumberList, "|"))

func indexOf(needle string, haystack []string) int {
	for i, v := range haystack {
		if v == needle {
			return i
		}
	}
	return -1
}

func main() {
	// Open file
	file, err := os.Open(gFilename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Read lines
	var totalSum uint64 = 0
	scanner := bufio.NewScanner(file)
	for lineCount := 1; scanner.Scan(); lineCount++ {
		line := scanner.Text()
		// Get first
		firstNum := gAllNumbersReg.FindString(line)
		if "" == firstNum {
			log.Fatalf("Failed to get firstNum\n")
		}
		// convert text words to int
		if idx := indexOf(firstNum, gNumberList); idx >= 0 {
			firstNum = fmt.Sprintf("%d", idx+1)
		}

		// Have to walk backwards to match overlapped
		secondNum := ""
		for lastIndex := len(line) - 1; "" == secondNum && lastIndex >= 0; lastIndex-- {
			secondNum = gAllNumbersReg.FindString(line[lastIndex:])
		}
		if "" == secondNum {
			log.Fatalf("secondNum to get firstNum\n")
		}
		if idx := indexOf(secondNum, gNumberList); idx >= 0 {
			secondNum = fmt.Sprintf("%d", idx+1)
		}

		// Merge
		// result, err := strconv.Atoi(firstNum + endNum)
		result, err := strconv.ParseUint(fmt.Sprintf("%s%s", firstNum, secondNum), 10, 64)
		if nil != err {
			log.Fatal(err)
		}
		totalSum += result
		fmt.Printf("#%2d: %d str(%s + %s) %v\n  = %d\n", lineCount, result, firstNum, secondNum, line, totalSum)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	// invalid: 53255
	fmt.Printf("Winner: %v", totalSum)
}
