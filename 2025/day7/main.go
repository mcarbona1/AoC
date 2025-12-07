package main

import (
	"fmt"
	"os"
	"slices"
	"strings"

	"github.com/akamensky/argparse"
)

func part1(input string) {
	splitters := map[int][]int{}
	beams := map[int]struct{}{}
	boardLen := 0

	for row, line := range strings.Split(input, "\n") {
		boardLen = len(line)
		splitterRow := []int{}
		for col, char := range line {
			switch char {
			case 'S':
				beams[col] = struct{}{}
			case '^':
				splitterRow = append(splitterRow, col)
			}
		}

		splitters[row] = splitterRow
	}

	splits := 0

	for i := range len(splitters) {
		splitterRow := splitters[i]

		for beam := range beams {
			if slices.Contains(splitterRow, beam) {
				splits += 1
				delete(beams, beam)

				if beam-1 >= 0 {
					beams[beam-1] = struct{}{}
				}

				if beam+1 < boardLen {
					beams[beam+1] = struct{}{}
				}
			}
		}
	}

	fmt.Printf("Splits: %d\n", splits)
}

func part2(input string) {
	splitters := map[int][]int{}
	beams := map[int]int{}
	boardLen := 0

	for row, line := range strings.Split(input, "\n") {
		boardLen = len(line)
		splitterRow := []int{}
		for col, char := range line {
			switch char {
			case 'S':
				beams[col] = 1
			case '^':
				splitterRow = append(splitterRow, col)
			}
		}

		splitters[row] = splitterRow
	}

	for i := range len(splitters) {
		splitterRow := splitters[i]

		for loc, weight := range beams {
			if slices.Contains(splitterRow, loc) {
				delete(beams, loc)

				if loc-1 >= 0 {
					beams[loc-1] += weight
				}

				if loc+1 < boardLen {
					beams[loc+1] += weight
				}
			}
		}
	}

	count := 0
	for _, w := range beams {
		count += w
	}

	fmt.Printf("Paths: %v\n", count)
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
