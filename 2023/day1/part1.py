#!/usr/bin/env python3

import sys

def main():
    sum_coord = 0
    for line in sys.stdin:
        line = line.strip()

        first = None
        second = None

        for char in line:
            if char.isdigit():
                if first is None:
                    first = char
                    second = char

                else:
                    second = char

        sum_coord += int(first+second)

    print(sum_coord)

if __name__ == "__main__":
    main()
