package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/akamensky/argparse"
)

func part1(input string) {
	location := 50
	count := 0
	for _, line := range strings.Split(input, "\n") {
		dir := line[0]
		amount, err := strconv.Atoi(line[1:])
		if err != nil {
			fmt.Printf("Failed to parse int from line: %s\n", line)
			return
		}

		if dir == 'L' {
			location = (location - amount) % 100
		} else if dir == 'R' {
			location = (location + amount) % 100
		}

		if location == 0 {
			count += 1
		}

	}

	fmt.Printf("Password: %d\n", count)
}

func part2(input string) {
	location := 50
	count := 0
	for line := range strings.SplitSeq(input, "\n") {
		dir := line[0]
		amount, err := strconv.Atoi(line[1:])
		if err != nil {
			fmt.Printf("Failed to parse int from line: %s\n", line)
			return
		}

		if dir == 'L' {
			if (location - amount) <= 0 {
				clicks := (location - amount) / -100
				if location != 0 {
					clicks += 1
				}
				count += clicks
			}

			// Weird work around to compensate for the fact that in go, modulo of a negative number will give a negative number
			location = ((location-amount)%100 + 100) % 100

		} else if dir == 'R' {
			clicks := (location + amount) / 100
			location = (location + amount) % 100

			count += clicks
		}

	}

	fmt.Printf("Password: %d\n", count)
}

func handle_cases(pt1 bool, pt2 bool, files []string) {
	for _, file := range files {
		content, err := os.ReadFile(file)
		if err != nil {
			fmt.Printf("Failed to read file: %s", file)
		}

		fmt.Printf("******* %s *******\n", file)

		if pt1 {
			fmt.Print("Part 1 ")
			part1(string(content))
		}

		if pt2 {
			fmt.Print("Part 2 ")
			part2(string(content))
		}

		fmt.Printf("***** %s end *****\n\n", file)
	}
}

func main() {
	parser := argparse.NewParser("parser", "Parsing")

	pt1 := parser.Flag("1", "pt1", nil)
	pt2 := parser.Flag("2", "pt2", nil)
	test := parser.Flag("t", "test", nil)
	input := parser.Flag("i", "input", nil)

	parser.Parse(os.Args)

	var files = []string{}

	if *test {
		files = append(files, "test.txt")
	}

	if *input {
		files = append(files, "input.txt")
	}

	handle_cases(*pt1, *pt2, files)
}
