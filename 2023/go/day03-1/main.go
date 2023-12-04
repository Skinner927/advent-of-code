package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"unicode/utf8"
)

func check(err error, ctx ...string) {
	if nil != err {
		if len(ctx) > 0 {
			log.Fatal(ctx, err)
		}
		log.Fatal(err)
	}
}

func ensure[T any](ctx ...string) func(T, error) T {
	return func(obj T, err error) T {
		check(err, ctx...)
		return obj
	}
}

func runeIsNumber(char rune) bool {
	return char >= '0' && char <= '9'
}

func maxNum[T int](a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func isValidPartNumber(startX int, startY int, length int, parts [][]bool) bool {
	endX := startX + length
	for x := startX; x < endX; x++ {
		for modY := -1; modY < 2; modY++ {
			// checks row above, current, below
			checkY := startY + modY
			if checkY < 0 {
				continue
			}
			for modX := -1; modX < 2; modX++ {
				// checks column left, current, right
				checkX := x + modX
				if checkX < 0 {
					continue
				}
				if parts[checkY][checkX] {
					return true
				}
			}
		}
	}
	return false
}

const doExample bool = false

func main() {
	// Open file
	var filename string
	if doExample {
		filename = "example.txt"
	} else {
		filename = "input.txt"
	}
	fmt.Printf("Readling file %s\n", filename)
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Parts are marked as true on the grid
	// parts[y][x]
	parts := make([][]bool, 1000)
	for i := range parts {
		parts[i] = make([]bool, 1000)
	}

	// Fill parts grid
	scanner := bufio.NewScanner(file)
	for y := 0; scanner.Scan(); y++ {
		line := scanner.Text()
		for x, val := range line {
			// A part is a symbol that isn't a number or dot
			if '.' != val && !runeIsNumber(val) {
				parts[y][x] = true
			}
		}
	}

	validPartSum := 0

	// Search for part numbers
	currentNum := make([]byte, 0, utf8.UTFMax*6)
	numberStart := -1
	_ = ensure[int64]("seek")(file.Seek(0, 0))
	scanner = bufio.NewScanner(file)
	for y := 0; scanner.Scan(); y++ {
		// Extra dot so our numbers always end before newline
		line := scanner.Text() + "."
		// clear
		currentNum = currentNum[:0]
		numberStart = -1
		for x, val := range line {
			// A part is a symbol that isn't a number or dot
			if runeIsNumber(val) {
				currentNum = utf8.AppendRune(currentNum, val)
				if -1 == numberStart {
					// start collecting
					numberStart = x
				}
			} else if -1 != numberStart {
				// done collecting
				partNumStr := string(currentNum)
				partNumber, err := strconv.Atoi(partNumStr)
				check(err, "Atoi(currentNum)")
				if isValidPartNumber(numberStart, y, len(partNumStr), parts) {
					validPartSum += partNumber
				}

				// clear
				currentNum = currentNum[:0]
				numberStart = -1
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	// invalid: 53255
	fmt.Printf("validPartSum: %v", validPartSum)
}
