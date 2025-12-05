package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/akamensky/argparse"
)

func part1(input string) {
	checking := false
	fresh := 0
	ranges := [][2]int{}

main:
	for line := range strings.SplitSeq(input, "\n") {
		if line == "" {
			checking = true

			continue
		}

		if !checking {
			split := strings.Split(line, "-")
			start, _ := strconv.Atoi(split[0])
			end, _ := strconv.Atoi(split[1])

			ranges = append(ranges, [2]int{start, end})
		} else {
			candidate, _ := strconv.Atoi(line)

			for _, rng := range ranges {
				if rng[0] <= candidate && candidate <= rng[1] {
					fresh++

					continue main
				}
			}
		}
	}

	fmt.Printf("Fresh ingredients: %d\n", fresh)
}

func remove(slice [][2]int, index int) [][2]int {
	if index+1 > len(slice)-1 {
		return slice[:index]
	} else {
		return append(slice[:index], slice[index+1:]...)
	}
}

func part2(input string) {
	fresh := 0
	ranges := [][2]int{}

main:
	for line := range strings.SplitSeq(input, "\n") {
		if line == "" {
			break
		}

		split := strings.Split(line, "-")
		start, _ := strconv.Atoi(split[0])
		end, _ := strconv.Atoi(split[1])

		// Construct ranges, combining if necessary.
		for i := 0; i < len(ranges); i++ {
			rng := ranges[i]

			if start >= rng[0] && end <= rng[1] {
				// Fully enclosed by another range, do nothing
				continue main
			}

			if start < rng[0] && end > rng[1] {
				// New range fully encloses another range, delete the enclosed range
				ranges = remove(ranges, i)
				i--
			}

			if start < rng[0] && end >= rng[0] && end <= rng[1] {
				ranges = remove(ranges, i)
				i--

				end = rng[1]
			}
			if start >= rng[0] && start <= rng[1] && end > rng[1] {
				ranges = remove(ranges, i)
				i--

				start = rng[0]
			}
		}

		ranges = append(ranges, [2]int{start, end})
	}

	// Calculate total number of fresh options.
	for _, rng := range ranges {
		fresh += rng[1] - rng[0] + 1
	}

	fmt.Printf("Fresh range total: %d\n", fresh)
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
