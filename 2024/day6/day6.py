#!/usr/bin/env python3

from collections import defaultdict
import typing

DIRECTION_MAP: dict[tuple[int, int]] = {
    "^": (0, -1),
    ">": (1, 0),
    "<": (-1, 0),
    "v": (0, 1),
}

TURN_MAP: dict[str] = {
    "^": ">",
    ">": "v",
    "<": "^",
    "v": "<",
}

def read_input(input: typing.TextIO) -> tuple[list[str], int, int]:
    board: list[str] = []
    start_x: int = 0
    start_y: int = 0

    line_num: int = 0
    while line := input.readline():
        line = line.strip()
        board.append(line)
        if "^" in line:
            start_y = line_num
            start_x = line.index("^")

        line_num += 1

    return board, start_x, start_y

def track_guard(board: list[str], start_x: int, start_y: int) -> int:
    total_spaces: int = 0

    x: int = start_x
    y: int = start_y

    h = len(board)
    l = len(board[0])

    curr_direction: str = "^"
    x_mod, y_mod = DIRECTION_MAP[curr_direction]

    visited: list[list[str]] = [[] for _ in range(h * l)]

    visited[y * l + x].append(curr_direction)
    total_spaces += 1

    while 0 <= x + x_mod < l and 0 <= y + y_mod < h:
        if board[y + y_mod][x + x_mod] == "#":
            curr_direction = TURN_MAP[curr_direction]
            x_mod, y_mod = DIRECTION_MAP[curr_direction]

            continue

        x += x_mod
        y += y_mod

        # Loop detected, return -1
        if curr_direction in visited[y * l + x]:
            return -1

        if not visited[y * l + x]:
            total_spaces += 1
            visited[y * l + x].append(curr_direction)


    return total_spaces

def generate_blocked_board(board: list[str], x: int, y: int):
    new_board = list()

    new_board += board[:y]
    new_line = (board[y][:x] + "#" + board[y][x+1:])
    new_board += [new_line]
    new_board += board[y+1:]

    return new_board

# Better to place obstacles only on path that was found in pt 1
def loop_guard(board: list[str], start_x: int, start_y: int):
    loops: int = 0
    h: int = len(board)
    l: int = len(board[0])

    for y in range(h):
        for x in range(l):
            if x == start_x and y == start_y:
                continue
            new_board: list[str] = generate_blocked_board(board, x, y)

            if track_guard(new_board, start_x, start_y) < 0:
                loops += 1

    return loops

def pt1():
    print("Part 1:")

    with open("example.txt") as input_file:
        board, x, y = read_input(input_file)
        total_spaces: int = track_guard(board, x, y)

        print(f"Total spaces for example: {total_spaces}")


    with open("input.txt") as input_file:
        board, x, y = read_input(input_file)
        total_spaces: int = track_guard(board, x, y)

        print(f"Total spaces for example: {total_spaces}")


    print("")

def pt2():
    print("Part 2:")
    for path_name in ["example", "input"]:
        with open(f"{path_name}.txt") as input_file:
            board, x, y = read_input(input_file)
            loops: int = loop_guard(board, x, y)

            print(f"Total blocked timelines for {path_name}: {loops}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()