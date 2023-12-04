package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

const gFilename = "input.txt"

var gStartNumReg = regexp.MustCompile(`^[^0-9]*([0-9])`)
var gEndNumReg = regexp.MustCompile(`([0-9])[^0-9]*$`)

func main() {
	// Open file
	file, err := os.Open(gFilename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Read lines
	total_sum := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		startNum, endNum := "", ""
		if startMatch := gStartNumReg.FindStringSubmatch(line); nil != startMatch {
			startNum = startMatch[1]
		}
		if endMatch := gEndNumReg.FindStringSubmatch(line); nil != endMatch {
			endNum = endMatch[1]
		}
		if "" == startNum || "" == endNum {
			log.Fatalf("Failed to find start %v or end %v for line %v\n", startNum, endNum, line)
		}
		result, err := strconv.Atoi(startNum + endNum)
		if nil != err {
			log.Fatal(err)
		}
		fmt.Println(result)
		total_sum += result
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Winner: %v", total_sum)
}
