#!/usr/bin/env python3

def read_input(path: str) -> list[str]:
    search: list[str] = []

    for line in open(path):
        search.append(line)

    return search

def find_xmas(search: list[str]) -> int:
    total: int = 0

    h = len(search)
    l = len(search[0])

    for i in range(h):
        for j in range(l):
            if search[i][j] != "X":
                continue
            for i_mod in range(-1,1+1):
                if (i_mod * 3 + i < 0 or i_mod * 3 + i >= h):
                    continue
                for j_mod in range(-1,1+1):
                    if (j_mod * 3 + j < 0 or j_mod * 3 + j >= h):
                        continue

                    if (j_mod == 0 and i_mod == 0):
                        continue

                    if search[i][j] + search[i + i_mod][j+j_mod] + search[i + i_mod*2][j+j_mod*2] + search[i + i_mod*3][j+j_mod*3] == "XMAS":
                        total += 1

    return total

def find_x_mas(search: list[str]) -> int:
    total: int = 0

    h = len(search)
    l = len(search[0])

    for i in range(1, h-1):
        for j in range(1, l-1):
            if search[i][j] != "A":
                continue
            diag_tl_br = search[i-1][j-1] + search[i][j] + search[i+1][j+1]
            diag_bl_tr = search[i+1][j-1] + search[i][j] + search[i-1][j+1]

            if (diag_bl_tr == "MAS" or diag_bl_tr == "SAM") and (diag_tl_br == "MAS" or diag_tl_br == "SAM"):
                total += 1

    return total

def pt1():
    print("Part 1:")

    search = read_input("./example.txt")

    print(f"Total xmas for example: {find_xmas(search)}")


    search = read_input("./input.txt")

    print(f"Total xmas for input: {find_xmas(search)}")

    print("")

def pt2():
    print("Part 2:")

    search = read_input("./example.txt")

    print(f"Total xmas for example: {find_x_mas(search)}")


    search = read_input("./input.txt")

    print(f"Total xmas for input: {find_x_mas(search)}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()