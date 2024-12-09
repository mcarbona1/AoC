#!/usr/bin/env python3

from collections import defaultdict
import typing

OPS: list[str] = ["+", "*"]
OPS_2: list[str] = ["||"]

def read_input(input: typing.TextIO) -> tuple[dict[str, list[tuple[int, int]]], int, int]:
    nodes: dict[str, list[tuple[int, int]]] = defaultdict(list)

    h: int = 0
    l: int = 0

    for line in input:
        h += 1

        line = line.strip()
        l = len(line)

        for x, char in enumerate(line):
            if char != ".":
                nodes[char].append((x, h - 1))

    return nodes, h, l

def antinode_valid(antinode: tuple[int, int], h: int , l: int) -> bool:
    return 0 <= antinode[0] < l and 0 <= antinode[1] < h

def find_antinodes(nodes: dict[str, list[tuple[int, int]]], h: int, l: int) -> int:
    antinodes: set[tuple[int, int]] = set()

    for node_type in nodes:
        node_list: list[tuple[int, int]] = nodes[node_type]

        while node_list:
            n1 = node_list.pop(0)

            for n2 in node_list:
                dx, dy = get_node_displacement(n1, n2)

                an1 = (n1[0] - dx, n1[1] - dy)
                an2 = (n2[0] + dx, n2[1] + dy)

                for an in [an1, an2]:
                    if antinode_valid(an, h, l):
                        antinodes.add(an)

    return len(antinodes)

def find_antinodes_pt2(nodes: dict[str, list[tuple[int, int]]], h: int, l: int) -> int:
    antinodes: set[tuple[int, int]] = set()

    for node_type in nodes:
        node_list: list[tuple[int, int]] = nodes[node_type]

        while node_list:
            n1 = node_list.pop(0)
            antinodes.add(n1)

            for n2 in node_list:
                dx, dy = get_node_displacement(n1, n2)

                an = (n1[0] - dx, n1[1] - dy)

                while antinode_valid(an, h, l):
                    antinodes.add(an)
                    an = (an[0] - dx, an[1] - dy)

                an = (n2[0] + dx, n2[1] + dy)
                while antinode_valid(an, h, l):
                    antinodes.add(an)
                    an = (an[0] + dx, an[1] + dy)

    return len(antinodes)

def get_node_displacement(n1: tuple[int, int], n2: tuple[int, int]):
    x1, y1 = n1
    x2, y2 = n2

    return x2 - x1, y2 -y1



def pt1():
    print("Part 1:")

    for file_base in [
                        "example",
                        "input",
                      ]:
        with open(f"{file_base}.txt") as input_file:
            nodes, h, l = read_input(input_file)

            num_antinodes: int = find_antinodes(nodes, h, l)

            print(f"Total number of antinodes for {file_base}: {num_antinodes}")

    print("")

# This can definitely be improved using pruning and evaluating as we recurse, if it gets bigger than the target, quit early
def pt2():
    print("Part 2:")

    for file_base in [
                        "example",
                        "input",
                      ]:
        with open(f"{file_base}.txt") as input_file:
            nodes, h, l = read_input(input_file)

            num_antinodes: int = find_antinodes_pt2(nodes, h, l)

            print(f"Total number of antinodes for {file_base}: {num_antinodes}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()