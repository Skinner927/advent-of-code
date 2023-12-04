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

const doExample bool = false

func main() {
	// Open file
	var filename string
	if doExample {
		filename = "../day02-1/example.txt"
	} else {
		filename = "../day02-1/input.txt"
	}
	fmt.Printf("Readling file %s\n", filename)
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Read lines
	totalSum := 0
	scanner := bufio.NewScanner(file)
	for lineNum := 1; scanner.Scan(); lineNum++ {
		// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
		line := scanner.Text()
		cubeRe := regexp.MustCompile(`([0-9]+) (red|green|blue)`)

		parts := strings.SplitN(line, ":", 2)
		gameSets := strings.Split(parts[1], ";")

		myMax := map[string]int{
			"red":   0,
			"green": 0,
			"blue":  0,
		}

		for _, allBlocks := range gameSets {
			for _, singleBlock := range strings.Split(allBlocks, ",") {
				match := cubeRe.FindStringSubmatch(singleBlock)
				if nil == match || len(match) < 3 {
					log.Fatalf("Failed block %v for line %v\n", singleBlock, line)
				}
				count := ensure[int]("singleBlock count")(strconv.Atoi(match[1]))
				// MAX
				if count > myMax[match[2]] {
					myMax[match[2]] = count
				}
			}
		}
		totalSum += myMax["red"] * myMax["green"] * myMax["blue"]
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	// invalid: 53255
	fmt.Printf("Winner: %v", totalSum)
}
