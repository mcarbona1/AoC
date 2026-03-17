package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"

	"github.com/akamensky/argparse"
)

type schematic struct {
	target  string
	buttons [][]int
	joltage []int
}

type node struct {
	state   string
	steps   []int
	options [][]int
}

func parseInput(input string) []schematic {
	parsed := []schematic{}
	for line := range strings.SplitSeq(input, "\n") {
		split := strings.Split(line, " ")

		buttons := [][]int{}

		for i, wiring := range split[1 : len(split)-1] {
			wires := strings.Split(wiring[1:len(wiring)-1], ",")

			buttons = append(buttons, []int{})

			for _, wire := range wires {
				index, _ := strconv.Atoi(wire)

				buttons[i] = append(buttons[i], index)
			}

		}

		joltage := []int{}
		joltages := split[len(split)-1][1 : len(split[len(split)-1])-1]
		for strNum := range strings.SplitSeq(joltages, ",") {
			num, _ := strconv.Atoi(strNum)
			joltage = append(joltage, num)
		}

		parsed = append(parsed, schematic{target: split[0], buttons: buttons, joltage: joltage})
	}

	return parsed
}

func joltageToTarget(joltages []int) string {
	target := ""
	for _, num := range joltages {
		if num%2 == 0 {
			target += "."
		} else {
			target += "#"
		}
	}

	return target
}

func pushButtons(input string, buttons []int) string {
	returnStr := ""
	for i, char := range input {
		if slices.Contains(buttons, i) {
			if char == '.' {
				returnStr += "#"
			} else {
				returnStr += "."
			}
		} else {
			returnStr += string(char)
		}
	}

	return returnStr
}

func solveSchematic(scm schematic) []int {
	target := scm.target[1 : len(scm.target)-1]
	start := ""
	for range len(target) {
		start = start + "."
	}
	frontier := []node{{state: start, steps: []int{}}}

	for {
		var n node
		n, frontier = frontier[0], frontier[1:]

		for i, btn := range scm.buttons {
			next := pushButtons(n.state, btn)

			if next == target {
				return append(n.steps, i)
			}

			frontier = append(frontier, node{state: next, steps: append(n.steps, i)})
		}
	}
}

func getAllOptions(scm schematic) [][]int {
	solutions := [][]int{}
	start := ""
	for range len(scm.target) {
		start = start + "."
	}
	frontier := []node{{state: start, steps: []int{}, options: scm.buttons}}

	for len(frontier) > 0 {
		var n node
		n, frontier = frontier[0], frontier[1:]

		for i, btn := range n.options {
			next := pushButtons(n.state, btn)

			if next == scm.target {
				solutions = append(solutions, append(n.steps, i))
				continue
			}

			var options [][]int
			options = append(options, n.options...)
			// fmt.Printf("Copied options: %v\n", options)
			nextOptions := append(options[:i], options[i+1:]...)
			// fmt.Printf("Next options: %v\n", nextOptions)
			// fmt.Printf("Part 1: %v\n", n.options[:i])
			// fmt.Printf("Part 2: %v\n", n.options[i+1:])
			frontier = append(frontier, node{state: next, steps: append(n.steps, i), options: nextOptions})
		}
	}

	return solutions
}

func solveJoltages(scm schematic) int {
	target := joltageToTarget(scm.joltage)
	options := getAllOptions(schematic{target: target, buttons: scm.buttons})
	fmt.Printf("Solutions: %v\n", options)

	for _, option := range options {
		// fmt.Printf("Potential solution:\n")
		for _, btn := range option {
			// fmt.Printf("BTN: %v\n", scm.buttons[btn])
			for _, idx := range scm.buttons[btn] {
				scm.joltage[idx] -= 1
			}
		}
	}

	return 0
}

func part1(input string) {
	schematics := parseInput(input)
	total := 0

	for _, scm := range schematics {
		steps := solveSchematic(scm)
		total += len(steps)
	}

	fmt.Printf("Total: %d\n", total)
}

func part2(input string) {
	schematics := parseInput(input)
	total := 0

	for _, scm := range schematics {
		total += solveJoltages(scm)
	}

	fmt.Printf("Total: %d\n", total)
}

func handleCases(pt1 bool, pt2 bool, files []string) {
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

	handleCases(*pt1, *pt2, files)
}
