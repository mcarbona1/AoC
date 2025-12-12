package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/akamensky/argparse"
)

type node struct {
	value   string
	visited []string
}

func buildGraph(input string) map[string][]string {
	graph := map[string][]string{}

	for line := range strings.SplitSeq(input, "\n") {
		split := strings.Split(line, " ")

		graph[split[0][:len(split[0])-1]] = split[1:]
	}

	return graph
}

func traverseGraph(graph map[string][]string) int {
	count := 0

	frontier := []string{"you"}

	for len(frontier) > 0 {
		var curr string
		curr, frontier = frontier[0], frontier[1:]

		if curr == "out" {
			count += 1

			continue
		}

		frontier = append(frontier, graph[curr]...)
	}

	return count
}

func traverseDACAndFFT(graph map[string][]string, cache map[string]int, curr string, target string) int {
	if num, found := cache[curr]; found {
		return num
	}

	if curr == target {
		return 1
	}

	total := 0

	for _, next := range graph[curr] {
		total += traverseDACAndFFT(graph, cache, next, target)
	}

	cache[curr] = total

	return total
}

func part1(input string) {
	graph := buildGraph(input)
	count := traverseGraph(graph)
	fmt.Printf("Paths out: %d\n", count)
}

func part2(input string) {
	graph := buildGraph(input)
	toFft := traverseDACAndFFT(graph, map[string]int{}, "svr", "fft")
	toDac := traverseDACAndFFT(graph, map[string]int{}, "fft", "dac")
	toOut := traverseDACAndFFT(graph, map[string]int{}, "dac", "out")

	fmt.Printf("DAC and FFT paths: %d\n", toFft*toDac*toOut)
}

func handleCases(pt1 bool, pt2 bool, files []string) {
	for _, file := range files {
		if file == "test.txt" && pt2 {
			continue
		} else if file == "test2.txt" && pt1 {
			continue
		}

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
		files = append(files, "test2.txt")
	}

	if *input {
		files = append(files, "input.txt")
	}

	handleCases(*pt1, *pt2, files)
}
