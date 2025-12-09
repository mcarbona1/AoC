package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"

	"github.com/akamensky/argparse"
)

type Point struct {
	x int
	y int
}

func part1(input string) {
	tiles := []Point{}
	for line := range strings.SplitSeq(input, "\n") {
		split := strings.Split(line, ",")
		x, _ := strconv.Atoi(split[0])
		y, _ := strconv.Atoi(split[1])

		tiles = append(tiles, Point{x: x, y: y})
	}

	maxArea := 0

	for _, pt := range tiles {
		for _, pt2 := range tiles {
			deltaX := int(math.Abs(float64(pt.x)-float64(pt2.x))) + 1
			deltaY := int(math.Abs(float64(pt.y)-float64(pt2.y))) + 1

			area := deltaX * deltaY

			if area > maxArea {
				maxArea = area
			}
		}
	}

	fmt.Printf("Max Area: %d\n", maxArea)
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func checkRect(pt1 Point, pt2 Point, tiles []Point) bool {
	minX := min(pt1.x, pt2.x)
	maxX := max(pt1.x, pt2.x)
	minY := min(pt1.y, pt2.y)
	maxY := max(pt1.y, pt2.y)

	p1 := tiles[0]

	for i := 1; i <= len(tiles); i++ {
		p2 := tiles[i%len(tiles)]

		if p1.x == p2.x {
			// Vertical.
			if p1.x > minX && p1.x < maxX {
				segMinY := min(p1.y, p2.y)
				segMaxY := max(p1.y, p2.y)

				if max(segMinY, minY) < min(maxY, segMaxY) {
					return false
				}
			}
		} else {
			// Horizontal.
			if p1.y > minY && p1.y < maxY {
				segMinX := min(p1.x, p2.x)
				segMaxX := max(p1.x, p2.x)

				if max(segMinX, minX) < min(maxX, segMaxX) {
					return false
				}
			}
		}

		p1 = p2
	}

	p1 = tiles[0]
	inside := false

	var centerX float32 = float32(minX+minX) / 2.0
	var centerY float32 = float32(minY+minY) / 2.0

	for i := 1; i <= len(tiles); i++ {
		p2 := tiles[i%len(tiles)]

		if p1.x == p2.x {
			edgeX := float32(p1.x)

			if edgeX > centerX {
				edgeMinY := float32(min(p1.y, p2.y))
				edgeMaxY := float32(max(p1.y, p2.y))

				if centerY > edgeMinY && centerY < edgeMaxY {
					inside = !inside
				}
			}
		}

		p1 = p2
	}

	return inside
}

func part2(input string) {
	redTiles := []Point{}
	for line := range strings.SplitSeq(input, "\n") {
		split := strings.Split(line, ",")
		x, _ := strconv.Atoi(split[0])
		y, _ := strconv.Atoi(split[1])

		redTiles = append(redTiles, Point{x: x, y: y})
	}

	maxArea := 0

	for _, pt := range redTiles {
		for _, pt2 := range redTiles {
			deltaX := int(math.Abs(float64(pt.x)-float64(pt2.x))) + 1
			deltaY := int(math.Abs(float64(pt.y)-float64(pt2.y))) + 1

			area := deltaX * deltaY

			// Check if it is contained in polygon
			if area > maxArea && checkRect(pt, pt2, redTiles) {
				maxArea = area
			}
		}
	}

	fmt.Printf("Max Area: %d\n", maxArea)
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
