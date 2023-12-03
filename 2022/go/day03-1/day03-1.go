package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("sample1.txt")
	if err != nil {
		log.Fatal(err)
	}
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
		fmt.Printf("Got line: %s\n", line)

		// Internet tells me empty structs use zero memory
		left := make(map[byte]struct{}, 26)
		right := make(map[byte]struct{}, 26)

		// Fill left and right
		sack := strings.NewReader(line)
		half := sack.Size() / 2
		var i int64 = 0
		for ; ; i += 1 {
			val, err := sack.ReadByte()
			if err != nil {
				if err != io.EOF {
					log.Fatal("Failed to read line: ", err)
				}
				break
			}
			if i < half {
				left[val] = struct{}{}
			} else {
				right[val] = struct{}{}
			}
		}

		// Find common
		var common byte = '!'
		for k, _ := range left {
			if _, has := right[k]; has {
				common = k
				break
			}
		}

		var commonVal uint8 = 0
		if common >= 'a' && common <= 'z' {
			commonVal = 'a' + ('z' - common)
		} else if common >= 'A' && common <= 'Z' {
			commonVal = 'A' + ('Z' - common)
		} else {
			log.Fatalf("common out of bounds %c\n", common)
		}
		fmt.Printf("Got %d for %c\n", commonVal, common)

	}

	fmt.Printf("\n\nourScore: %d\n", ourScore)
}
