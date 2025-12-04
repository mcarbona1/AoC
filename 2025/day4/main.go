package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/akamensky/argparse"
)

func checkSquare(mapping []string, row int, col int, height int, width int, gen byte) bool {
	count := 0

	for i := row - 1; i <= row+1; i += 1 {
		for j := col - 1; j <= col+1; j += 1 {
			if i == row && j == col {
				continue
			} else if i < 0 || j < 0 || i >= height || j >= width {
				continue
			} else if mapping[i][j] == '@' || mapping[i][j] == gen {
				count += 1
			}
		}
	}

	return count < 4
}

func part1(input string) {
	mapping := strings.Split(input, "\n")
	count := 0

	height := len(mapping)
	width := len(mapping[0])

	for i, row := range mapping {
		for j, char := range row {
			if char != '@' {
				continue
			}

			// The generation doesn't matter here as long as it's not '.'
			if checkSquare(mapping, i, j, height, width, '@') {
				count += 1
			}
		}
	}

	fmt.Printf("Accessible rolls: %d\n", count)
}

func part2(input string) {
	var generation byte = 1

	count := 0
	mapping := strings.Split(input, "\n")
	removed := true

	height := len(mapping)
	width := len(mapping[0])

	for removed {
		if generation == '.' || generation == '@' {
			generation += 1

			continue
		}

		removed = false

		for i, row := range mapping {
			for j, char := range row {
				if char != '@' {
					continue
				}

				if checkSquare(mapping, i, j, height, width, generation) {
					count += 1
					removed = true

					// Mark roll as deleted for future generations. This technically does have a maximum limit for generations but for this sample size it works.
					mapping[i] = mapping[i][:j] + fmt.Sprintf("%c", generation) + mapping[i][j+1:]
				}
			}
		}

		generation += 1
	}

	fmt.Printf("Accessible rolls: %d\n", count)
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
