package main

import (
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"

	"AoC/2025/collections"

	"github.com/akamensky/argparse"
)

type Point struct {
	x int
	y int
	z int
}

type Pair struct {
	p1 Point
	p2 Point
}

func (pt Point) distance(pt2 Point) float64 {
	return math.Sqrt(math.Pow(float64(pt2.x)-float64(pt.x), 2) + math.Pow(float64(pt2.y)-float64(pt.y), 2) + math.Pow(float64(pt2.z)-float64(pt.z), 2))
}

func part1(input string, numPts int) {
	points := []Point{}
	for line := range strings.SplitSeq(input, "\n") {
		coords := strings.Split(line, ",")

		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		z, _ := strconv.Atoi(coords[2])

		points = append(points, Point{x, y, z})
	}

	pq := collections.PriorityQueue[Pair, float64]{}

	for i, pt := range points {
		for j, pt2 := range points {
			if j <= i {
				continue
			}

			dist := pt.distance(pt2)

			pq.PushRaw(Pair{pt, pt2}, dist)

		}
	}

	pq.Sort()

	circuits := map[Point]*[]Point{}

	circuitLens := []int{}
	processed := 0

	for processed < numPts {
		processed++

		pair, _ := pq.Pop()

		c1 := circuits[pair.p1]
		c2 := circuits[pair.p2]

		if c1 == nil && c2 == nil {
			circuit := []Point{pair.p1, pair.p2}

			circuits[pair.p1] = &circuit
			circuits[pair.p2] = &circuit

			c1 = &circuit
			c2 = &circuit

		} else if c1 == nil {
			*c2 = append(*c2, pair.p1)

			circuits[pair.p1] = c2
			c1 = c2
		} else if c2 == nil {
			*c1 = append(*c1, pair.p2)

			circuits[pair.p2] = c1
			c2 = c1
		} else if c1 != c2 {
			*c1 = append(*c1, *c2...)

			for _, pt := range *c2 {
				circuits[pt] = c1
			}
		}
	}

	seen := map[*[]Point]struct{}{}

	for _, value := range circuits {
		if _, found := seen[value]; found {
			continue
		}

		circuitLens = append(circuitLens, len(*value))
		seen[value] = struct{}{}
	}

	slices.Sort(circuitLens)
	slices.Reverse(circuitLens)

	total := 1

	for _, ln := range circuitLens[:3] {
		total *= ln
	}

	fmt.Printf("Top 3 circuit lens multiplied: %d\n", total)
}

func part2(input string) {
	points := []Point{}
	for line := range strings.SplitSeq(input, "\n") {
		coords := strings.Split(line, ",")

		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		z, _ := strconv.Atoi(coords[2])

		points = append(points, Point{x, y, z})
	}

	targetLen := len(points)

	pq := collections.PriorityQueue[Pair, float64]{}

	for i, pt := range points {
		for j, pt2 := range points {

			if j <= i {
				continue
			}

			dist := pt.distance(pt2)

			pq.PushRaw(Pair{pt, pt2}, dist)
		}

	}

	pq.Sort()

	circuits := map[Point]*[]Point{}

	answer := 0

	for {
		pair, _ := pq.Pop()

		c1 := circuits[pair.p1]
		c2 := circuits[pair.p2]

		if c1 == nil && c2 == nil {
			circuit := []Point{pair.p1, pair.p2}

			circuits[pair.p1] = &circuit
			circuits[pair.p2] = &circuit

			c1 = &circuit
			c2 = &circuit

		} else if c1 == nil {
			*c2 = append(*c2, pair.p1)

			circuits[pair.p1] = c2
			c1 = c2
		} else if c2 == nil {
			*c1 = append(*c1, pair.p2)

			circuits[pair.p2] = c1
			c2 = c1
		} else if c1 != c2 {

			*c1 = append(*c1, *c2...)

			for _, pt := range *c2 {
				circuits[pt] = c1
			}
			c2 = c1
		} else if c1 == c2 {
			continue
		}

		if len(*c1) == targetLen {
			answer = pair.p1.x * pair.p2.x
			break
		}
	}

	fmt.Printf("p1 x * p2 x: %d\n", answer)
}

func handle_cases(pt1 bool, pt2 bool, files []string) {
	for _, file := range files {
		content, err := os.ReadFile(file)
		if err != nil {
			fmt.Printf("Failed to read file: %s", file)
		}

		fmt.Printf("******* %s *******\n", file)

		if pt1 {
			num := 1000

			if file == "test.txt" {
				num = 10
			}

			fmt.Print("Part 1 ")
			part1(string(content), num)
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
