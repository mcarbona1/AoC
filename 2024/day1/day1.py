#!/usr/bin/env python3

from collections import defaultdict

def read_input(path: str) -> tuple[list[int], list[int]]:
    left, right = list(), list()

    for line in open(path):
        line = line.strip()
        lNum, rNum = [int(num) for num in line.split("   ")]
        left.append(lNum)
        right.append(rNum)

    return (sorted(left), sorted(right))

def get_list_diff(left, right) -> int:
    total = 0
    for lNum, rNum in zip(left, right):
        total += abs(lNum - rNum)

    return total

def get_similarity(left, right) -> int:
    sim = 0
    right_counts = defaultdict(int)

    for num in right:
        right_counts[num] += 1

    for num in left:
        sim += num * right_counts[num]

    return sim

def pt1():
    print("Part 1:")
    left, right = read_input("./example.txt")
    print(f"Total disparity for example: {get_list_diff(left, right)}")

    left, right = read_input("./input.txt")
    print(f"Total disparity for input: {get_list_diff(left, right)}")

    print("")

def pt2():
    print("Part 2:")
    left, right = read_input("./example.txt")
    print(f"Similarity for example: {get_similarity(left, right)}")

    left, right = read_input("./input.txt")
    print(f"Similarity for input: {get_similarity(left, right)}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()