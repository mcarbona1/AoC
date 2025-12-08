package main

import (
	"container/heap"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"

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

// An Item is something we manage in a priority queue.
type Item struct {
	value    Pair    // The value of the item; arbitrary.
	priority float64 // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the lowest, not highest, priority so we use less than here.
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // don't stop the GC from reclaiming the item eventually
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// update modifies the priority and value of an Item in the queue.
func (pq *PriorityQueue) Update(item *Item, value Pair, priority float64) {
	item.value = value
	item.priority = priority
	heap.Fix(pq, item.index)
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

	pq := make(PriorityQueue, len(points)*len(points))

	for i, pt := range points {
		for j, pt2 := range points {
			if j <= i {
				pq[i*len(points)+j] = &Item{value: Pair{pt, pt2}, priority: math.MaxFloat64, index: i*len(points) + j}
				continue
			}

			dist := pt.distance(pt2)

			pq[i*len(points)+j] = &Item{value: Pair{pt, pt2}, priority: dist, index: i*len(points) + j}
		}
	}

	heap.Init(&pq)

	circuits := map[Point]*[]Point{}

	circuitLens := []int{}
	processed := 0

	for processed < numPts {
		processed++

		item := heap.Pop(&pq).(*Item)

		pair := item.value

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

	pq := make(PriorityQueue, len(points)*len(points))

	for i, pt := range points {
		for j, pt2 := range points {

			if j <= i {
				pq[i*len(points)+j] = &Item{value: Pair{pt, pt2}, priority: math.MaxFloat64, index: i*len(points) + j}
				continue
			}

			dist := pt.distance(pt2)

			pq[i*len(points)+j] = &Item{value: Pair{pt, pt2}, priority: dist, index: i*len(points) + j}
		}

	}

	heap.Init(&pq)

	circuits := map[Point]*[]Point{}

	answer := 0

	for {
		item := heap.Pop(&pq).(*Item)

		pair := item.value

		c1 := circuits[pair.p1]
		c2 := circuits[pair.p2]

		if c1 == nil && c2 == nil {
			circuit := []Point{pair.p1, pair.p2}

			circuits[pair.p1] = &circuit
			circuits[pair.p2] = &circuit

			c1 = &circuit
			c2 = &circuit

		} else if c1 == nil {
			// fmt.Printf("c2 value: %v\n", *c2)
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
