package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/akamensky/argparse"
)

func checkInvalidPart1(num int) bool {
	str := strconv.Itoa(num)

	if len(str)%2 == 1 {
		return false
	}

	if str[:len(str)/2] != str[len(str)/2:] {
		return false
	}

	return true
}

func checkInvalidPart2(num int) bool {
	str := strconv.Itoa(num)

newSize:
	for size := 1; size <= len(str)/2; size += 1 {
		if len(str)%size != 0 {
			continue
		}

		check := str[0:size]

		for i := size; i < len(str) && i+size <= len(str); i += size {
			if str[i:i+size] != check {
				continue newSize
			}
		}
		return true
	}

	return false

}

func part1(input string) {
	total := 0

	for numRange := range strings.SplitSeq(input, ",") {
		strRange := strings.Split(numRange, "-")
		start, _ := strconv.Atoi(strRange[0])
		end, _ := strconv.Atoi(strRange[1])

		end += 1

		for i := start; i <= end; i += 1 {
			if checkInvalidPart1(i) {
				total += i
			}
		}
	}

	fmt.Printf("Invalid numbers total: %d\n", total)
}

func part2(input string) {
	total := 0

	for numRange := range strings.SplitSeq(input, ",") {
		strRange := strings.Split(numRange, "-")
		start, _ := strconv.Atoi(strRange[0])
		end, _ := strconv.Atoi(strRange[1])

		end += 1

		for i := start; i <= end; i += 1 {
			if checkInvalidPart2(i) {
				total += i
			}
		}
	}

	fmt.Printf("Invalid numbers total: %d\n", total)
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
