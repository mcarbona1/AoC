package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/akamensky/argparse"
)

func solveProblems(problems [][]int, ops []string) int {
	total := 0
	for i := range len(ops) {
		subtotal := problems[0][i]

		for _, row := range problems[1:] {
			if ops[i] == "+" {
				subtotal += row[i]
			} else {
				subtotal *= row[i]
			}
		}

		total += subtotal

	}

	return total
}

func solveProblems2(problems [][]int, ops []string) int {
	total := 0

	for i, problem := range problems {
		subtotal := problem[0]
		op := ops[i]

		for _, num := range problem[1:] {
			if op == "+" {
				subtotal += num
			} else {
				subtotal *= num
			}
		}

		total += subtotal
	}

	return total
}

func part1(input string) {
	problems := [][]int{}
	ops := []string{}

	for line := range strings.SplitSeq(input, "\n") {
		problem := []int{}

		for token := range strings.SplitSeq(line, " ") {
			if token == "" {
				continue
			}

			num, err := strconv.Atoi(token)

			if err == nil {
				problem = append(problem, num)
			} else {
				ops = append(ops, token)
			}
		}

		if len(problem) > 0 {
			problems = append(problems, problem)
		}
	}

	fmt.Printf("Answer: %d\n", solveProblems(problems, ops))
}

func part2(input string) {
	lines := strings.Split(input, "\n")

	ops := []string{}
	problems := [][]int{}

	problem := []int{}
	for i := len(lines[0]) - 1; i >= 0; i-- {
		strNum := ""

		for _, line := range lines {
			char := string(line[i])

			if char == " " {
				continue
			} else if char == "*" || char == "+" {
				ops = append(ops, char)
			} else {
				strNum += char
			}
		}

		if strNum == "" {
			problems = append(problems, problem)
			problem = []int{}
		} else {
			num, err := strconv.Atoi(strNum)

			if err != nil {
				fmt.Printf("Failed to convert: %s\n", strNum)
			} else {
				problem = append(problem, num)
			}

			if i == 0 {
				problems = append(problems, problem)
			}
		}
	}

	fmt.Printf("Answer: %d\n", solveProblems2(problems, ops))
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
