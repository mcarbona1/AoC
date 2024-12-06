#!/usr/bin/env python3

from collections import defaultdict
import re

def read_input(path: str) -> list[list[int]]:
    muls: list[list[int]] = []
    muls_re = re.compile(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\))")

    for line in open(path):
        muls += muls_re.findall(line)

    return muls

def calculate_muls(muls: list[tuple[str,str]], pt2: bool = False) -> int:
    total: int = 0
    enabled: bool = True

    for mul in muls:
        if "don't" in mul[0]:
            if pt2:
                enabled = False
            else:
                continue
        elif "do" in mul[0]:
            if pt2:
                enabled = True
            else:
                continue

        else:
            l: int = int(mul[1])
            r: int = int(mul[2])

            if enabled:
                total += l * r

    return total

def pt1():
    print("Part 1:")

    muls = read_input("./example.txt")

    print(f"Total muls for example: {calculate_muls(muls)}")


    muls = read_input("./input.txt")

    print(f"Total muls for input: {calculate_muls(muls)}")

    print("")

def pt2():
    print("Part 2:")

    muls = read_input("./example.txt")

    print(f"Total muls for example: {calculate_muls(muls, True)}")


    muls = read_input("./input.txt")

    print(f"Total muls for input: {calculate_muls(muls, True)}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()