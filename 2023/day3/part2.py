#!/usr/bin/env python3

import sys
from typing import List

def scan_int(x: int, y: int, schematic: List[str]) -> (int, int):
    board_len = len(schematic[y])
    i = x
    while i >= 0 and schematic[y][i].isdigit():

        i -= 1

    l_bound = i + 1

    i = x
    while i < board_len and schematic[y][i].isdigit():
        i += 1

    r_bound = i - 1


    return (l_bound, r_bound)

def scan_scematic(schematic: List[str]):
    x_bound = len(schematic[0])
    schematic_sum = 0
    for j in range(len(schematic)):
        for i in range(x_bound):
            if schematic[j][i] == "*":
                n1, n2 = None, None

                for y in range(j-1, j+2):
                    x = i-1
                    while x <= i + 1:
                        if schematic[y][x].isdigit():
                            l, r = scan_int(x, y, schematic)
                            if not n1:
                                n1 = int(schematic[y][l:r+1])
                            else:
                                n2 = int(schematic[y][l:r+1])

                            x = r + 1
                        else:
                            x += 1

                if n1 is not None and n2 is not None:
                    schematic_sum += n1 * n2

    return schematic_sum


def main():
    schematic = []
    for line in sys.stdin:
        schematic.append(line.strip())

    result = scan_scematic(schematic)
    print(result)

if __name__ == "__main__":
    main()
