#!/usr/bin/env python3

from collections import defaultdict

def read_input(path: str) -> list[list[int]]:
    levels: list[list[int]] = []

    for line in open(path):
        line = line.strip()
        readings: list[int] = [int(num) for num in line.split(" ")]
        levels.append(readings)

    return levels

def determine_safety(readings: list[int]) -> bool:
    safe: bool = True

    inc: bool = readings[0] < readings[1]

    last_reading = readings[0]

    for reading in readings[1:]:
        safe &= last_reading < reading if inc else last_reading > reading
        safe &= 1 <= abs(last_reading - reading) <= 3

        last_reading = reading

    return safe

def determine_dampening_safety(readings: list[int]) -> bool:
    for i in range(len(readings)):
        if determine_safety(readings[:i] + readings[i+1:]):
            return True

    return False
# def determine_dampening_safety(readings: list[int]) -> bool:
#     safe: bool = True
#     dampen: bool = True

#     inc: bool = readings[0] < readings[1]

#     last_reading = readings[0]

#     for reading in readings[1:]:
#         if not last_reading < reading if inc else last_reading > reading or not 1 <= abs(last_reading - reading) <= 3:
#             if dampen:
#                 dampen = False
#                 continue
#             else:
#                 safe = False
#                 break

#         last_reading = reading

#    return safe



def pt1():
    print("Part 1:")

    safety: int = 0
    levels = read_input("./example.txt")

    for readings in levels:
        safety += int(determine_safety(readings))

    print(f"Total safety for example: {safety}")

    safety: int = 0
    levels = read_input("./input.txt")

    for readings in levels:
        safety += int(determine_safety(readings))

    print(f"Total safety for input: {safety}")


    print("")

def pt2():
    print("Part 2:")

    safety: int = 0
    levels = read_input("./example.txt")

    for readings in levels:
        safety += int(determine_dampening_safety(readings))

    print(f"Total dampening safety for example: {safety}")

    safety: int = 0
    levels = read_input("./input.txt")

    for readings in levels:
        safety += int(determine_dampening_safety(readings))

    print(f"Total dampening safety for input: {safety}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()