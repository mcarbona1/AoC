package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/akamensky/argparse"
)

func part1(input string) {
	total := 0
	for line := range strings.SplitSeq(input, "\n") {
		var first *byte = nil
		var second *byte = nil

		for i := range len(line) {
			char := line[i]

			if i != len(line)-1 && (first == nil || char > *first) {
				first = &char
				second = nil
			} else if i != 0 && (second == nil || char > *second) {
				second = &char
			}
		}

		str := string([]byte{*first, *second})
		val, _ := strconv.Atoi(str)

		total += val
	}

	fmt.Printf("output joltage: %d\n", total)
}

func check_candidate(current string, candidate byte, length int, position int) string {
	const batteries = 12

	for i := range current {
		if candidate > current[i] && (length-position) >= (batteries-i) {
			return current[:i] + string(candidate)
		}
	}

	if len(current) < batteries {
		return current + string(candidate)
	}

	return current
}

func part2(input string) {
	total := 0
	for line := range strings.SplitSeq(input, "\n") {
		lineLen := len(line)

		current := ""

		for i := range lineLen {
			candidate := line[i]

			current = check_candidate(current, candidate, lineLen, i)
		}

		val, _ := strconv.Atoi(current)
		total += val

	}
	fmt.Printf("output joltage: %d\n", total)
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
