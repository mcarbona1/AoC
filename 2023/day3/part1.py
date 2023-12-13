#!/usr/bin/env python3

import sys
from typing import List

def scan_range(x_l: int, x_r: int, y: int, schematic: List[str]) -> bool:
    x_bound = len(schematic[0])
    y_bound = len(schematic)
    for j in range(y - 1, y + 2):
        for i in range (x_l - 1, x_r+2):
            if j < 0 or i < 0 or j >= y_bound - 1 or i >= x_bound:
                continue

            if (schematic[j][i] != '.' and not schematic[j][i].isdigit()):
                return True


    return False

def scan_scematic(schematic: List[str]):
    x_bound = len(schematic[0])
    schematic_sum = 0
    for j in range(len(schematic)):
        i = 0
        while i < x_bound:
            if schematic[j][i].isdigit():
                x_l = (i := i+1) - 1
                x_r = x_l
                while i < x_bound and schematic[j][i].isdigit():
                    x_r = i
                    i += 1
                if scan_range(x_l, x_r, j, schematic):
                    print(schematic[j][x_l:x_r+1])
                    schematic_sum += int(schematic[j][x_l:x_r+1])
                i -= 1

            i += 1

    return schematic_sum


def main():
    schematic = []
    for line in sys.stdin:
        schematic.append(line.strip())

    result = scan_scematic(schematic)
    print(result)

if __name__ == "__main__":
    main()
