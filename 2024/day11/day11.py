#!/usr/bin/env python3

import typing

from collections import defaultdict

NUM_BLINKS: dict[str, int] = {
    "example": 1,
    "large_example": 6,
    "input": 25,
}

def read_input(input: typing.TextIO) -> dict[str, int]:
    stones: dict[str, int] = defaultdict(int)

    for line in input:
        line = line.strip()

        for stone in line.split(" "):
            stones[stone] += 1


    return stones

def process_blinks(stones: dict[str, int], num_blinks: int) -> int:
    for _ in range(num_blinks):
        stones = apply_rules(stones)

    return sum(stones.values())

def apply_rules(stones: dict[str, int]) -> dict[str, int]:
    new_stones: dict[str, int] = defaultdict(int)
    key: str

    for stone in stones:
        if stone == "0":
            key = "1"
        elif len(stone) % 2 == 0:
            new_stones[str(int(stone[:len(stone)//2]))] += stones[stone]

            key = str(int(stone[len(stone)//2:]))
        else:
            key = str(int(stone) * 2024)

        new_stones[key] += stones[stone]

    return new_stones

def pt1():
    print("Part 1:")

    for file_base in [
                        "example",
                        "large_example",
                        "input",
                      ]:
        with open(f"{file_base}.txt") as input_file:
            stones: list[str] = read_input(input_file)

            num_stones: int = process_blinks(stones, NUM_BLINKS[file_base])

            print(f"Total number of trails for {file_base}: {num_stones}")

    print("")

def pt2():
    print("Part 2:")

    for file_base in [
                        "input",
                      ]:
        with open(f"{file_base}.txt") as input_file:
            stones: list[str] = read_input(input_file)

            num_stones: int = process_blinks(stones, 75)

            print(f"Total number of trails for {file_base}: {num_stones}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()