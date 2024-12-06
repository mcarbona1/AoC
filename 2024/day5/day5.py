#!/usr/bin/env python3

from collections import defaultdict
import typing

def read_input(input: typing.TextIO) -> dict[list[str]]:
    graph: dict[list[str]] = defaultdict(list)
    inverse: dict[list[str]] = defaultdict(list)

    while line := input.readline():
        line = line.strip()
        if line == "":
            break

        line = line.split("|")
        before, after = line[0], line[1]

        graph[before].append(after)
        inverse[after].append(before)

    return graph, inverse

def find_valid_orders(graph: dict[list[str]], input: typing.TextIO) -> list[list[str]]:
    valid_orders: list[list[str]] = []
    invalid_orders: list[list[str]] = []

    while line := input.readline():
        line = line.strip()
        ordering = line.split(",")

        seen: set[str] = set()

        valid: bool = True

        for page in ordering:
            for follow in graph[page]:
                if follow in seen:
                    valid = False
                    break

            seen.add(page)

        if valid:
            valid_orders.append(ordering)
        else:
            invalid_orders.append(ordering)

    return valid_orders, invalid_orders

def add_valid_middles(valid_orders: list[str]) -> int:
    total: int = 0

    for order in valid_orders:
        total += int(order[len(order)//2])

    return total

# This is terrible and should be a topological sort lmao
def reorder_invalid_orders(graph: dict[list[str]], inverse_graph: dict[list[str]], invalid_orders: list[list[str]]) -> list[list[str]]:
    for order in invalid_orders:
        seen: set[str] = set()

        changed = False

        i = 0
        while i  < len(order):
            for page in graph[order[i]]:
                if page in seen:
                    start: int = 0
                    end: int = len(order)

                    for page in graph[order[i]]:
                        try:
                            index = order.index(page)
                            end = index if index < end else end
                        except ValueError:
                            continue

                    for page in inverse_graph[order[i]]:
                        try:
                            index = order.index(page)
                            start = index if index > start else start
                        except ValueError:
                            continue

                    new_index = start + 1 if start + 1 < end else end
                    order.insert(new_index, order.pop(i))

                    changed = True

                    break

            seen.add(order[i])
            i += 1

            if changed:
                changed = False
                seen.clear()
                i = 0

    return invalid_orders


def pt1():
    print("Part 1:")

    with open("example.txt") as input_file:
        graph, _ = read_input(input_file)
        valid_orders, _ = find_valid_orders(graph, input_file)
        middle_total = add_valid_middles(valid_orders)

    print(f"Total middles for example: {middle_total}")


    with open("input.txt") as input_file:
        graph, _ = read_input(input_file)
        valid_orders, _ = find_valid_orders(graph, input_file)
        middle_total = add_valid_middles(valid_orders)

    print(f"Total middles for input: {middle_total}")

    print("")

def pt2():
    print("Part 2:")

    with open("example.txt") as input_file:
        graph, inverse_graph = read_input(input_file)
        _, invalid_orders = find_valid_orders(graph, input_file)
        invalid_orders = reorder_invalid_orders(graph, inverse_graph, invalid_orders)
        middle_total = add_valid_middles(invalid_orders)

    print(f"Total middles for example: {middle_total}")

    with open("input.txt") as input_file:
        graph, inverse_graph = read_input(input_file)
        _, invalid_orders = find_valid_orders(graph, input_file)
        invalid_orders = reorder_invalid_orders(graph, inverse_graph, invalid_orders)
        middle_total = add_valid_middles(invalid_orders)

    print(f"Total middles for input: {middle_total}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()