package main

import (
	"fmt"
	"os"

	"github.com/akamensky/argparse"
)

func part1(input string) bool {
	fmt.Printf("read file content:\n%s\n", input)
	return false
}

func part2(input string) bool {
	fmt.Printf("read file content:\n%s\n", input)
	return false
}

func handle_cases(pt1 bool, pt2 bool, files []string) {
	for _, file := range files {
		content, err := os.ReadFile(file)
		if err != nil {
			fmt.Printf("Failed to read file: %s", file)
		}

		if pt1 {
			part1(string(content))
		}

		if pt2 {
			part2(string(content))
		}
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
