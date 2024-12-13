#!/usr/bin/env python3

import typing

def read_input(input: typing.TextIO) -> tuple[list[list[int]], list[tuple[int, int]]]:
    trailmap: list[list[int]] = list()
    trailheads: list[tuple[int, int]] = list()

    for y, line in enumerate(input):

        line = line.strip()

        trailmap.append(list())

        for x, grade in enumerate(line):
            grade = int(grade)
            if grade == 0:
                trailheads.append((x, y))
            trailmap[-1].append(grade)

    return trailmap, trailheads

def find_tails(trailmap: list[list[int]], trailheads: list[tuple[int ,int]], pt1: bool = True) -> int:
    trails: int = 0
    for trailhead in trailheads:
        trails += find_trails_from_trailhead(trailmap, trailhead, pt1)

    return trails

def find_trails_from_trailhead(trailmap: list[list[int]], trailhead: tuple[int, int], pt1: bool = True) -> int:
    trails: int = 0
    visited: set[tuple[int, int, int]]= set()
    frontier: list[tuple[int, int, int]] = list()

    frontier.append((0, trailhead[0], trailhead[1]))

    while frontier:
        step = frontier.pop(0)
        if step in visited and pt1:
            continue

        if pt1:
            visited.add(step)

        curr_elevation, x, y = step


        if curr_elevation == 9:
            trails += 1
            continue

        steps: list[tuple[int, int, int]] = get_next_step(trailmap, curr_elevation, x, y)

        for step in steps:
            frontier.append(step)

    return trails

def get_next_step(trailmap: list[list[int]], curr_elevation: int, x: int, y: int) -> list[tuple[int, int, int]]:
    l: int = len(trailmap[0])
    h: int = len(trailmap)

    next_steps: list[tuple[int, int, int]] = list()

    for x_mod in range(-1, 1 + 1):
        for y_mod in range(-1, 1 + 1):
            if not (0 <= x + x_mod < l and 0 <= y + y_mod < h) or (x_mod != 0 and y_mod != 0):
                continue

            if trailmap[y + y_mod][x + x_mod] == curr_elevation + 1:
                next_steps.append((curr_elevation+1, x + x_mod, y + y_mod))

    return next_steps

def pt1():
    print("Part 1:")

    for file_base in [
                        "example",
                        "large_example",
                        "input",
                      ]:
        with open(f"{file_base}.txt") as input_file:
            trailmap, trailheads = read_input(input_file)

            trails: int = find_tails(trailmap, trailheads)

            print(f"Total number of trails for {file_base}: {trails}")

    print("")

def pt2():
    print("Part 2:")

    for file_base in [
                        "example",
                        "large_example",
                        "input",
                      ]:
        with open(f"{file_base}.txt") as input_file:
            trailmap, trailheads = read_input(input_file)

            trails: int = find_tails(trailmap, trailheads, False)

            print(f"Total number of trails for {file_base}: {trails}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()