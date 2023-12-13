#!/usr/bin/env python3

import sys

mapping = {'one': '1',
           'two': '2',
           'three': '3',
           'four': '4',
           'five': '5',
           'six': '6',
           'seven': '7',
           'eight': '8',
           'nine': '9',
           'zero': '0'
           }

def is_number(string: str):
    for num in mapping:
        if string.startswith(num):
            return mapping[num]

    return None

def main():
    sum_coord = 0
    for line in sys.stdin:
        line = line.strip()

        first = None
        second = None

        for i in range(len(line)):
            char = line[i]
            if char.isdigit() or (char := is_number(line[i:])):
                if first is None:
                    first = char
                    second = char

                else:
                    second = char

        sum_coord += int(first+second)

    print(sum_coord)

if __name__ == "__main__":
    main()
