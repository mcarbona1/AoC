#!/usr/bin/env python3

from collections import defaultdict
import typing

OPS: list[str] = ["+", "*"]
OPS_2: list[str] = ["||"]

def read_input(input: typing.TextIO) -> list[tuple[int, list[int]]]:
    transactions: list[tuple[int, list[int]]] = []

    for line in input:
        line = line.strip()
        target, numbers = line.split(": ")
        transactions.append((int(target), [int(num) for num in numbers.split(" ")]))

    return transactions

def validate_transaction(target: int, nums: list[int], base: str = "", pt2: bool = False) -> int:
    valid: int = 0

    if nums:
        num = nums.pop(0)
        for op in OPS + OPS_2 if pt2 else []:
            valid += validate_transaction(target, nums, f"{base} {op} {num}" if base else f"{num}", pt2)

        nums.insert(0, num)
    # Base case
    else:
        #print(base)
        if evaluate_expr(base) == target:
            valid += 1

    return valid

def evaluate_expr(expr: str) -> int:
    total: int = 0

    terms = expr.split(" ")

    op = "+"

    total += int(terms[0])

    for term in terms[1:]:
        if term in OPS + OPS_2:
            op = term
        else:
            num = int(term)

            if op == "*":
                total *= num
            elif op == "+":
                total += num
            else:
                total = int(str(total) + str(num))

    return total

def pt1():
    print("Part 1:")

    for file_base in ["example", "input"]:
        with open(f"{file_base}.txt") as input_file:
            transactions = read_input(input_file)

            transactions_total: int = 0

            for transaction in transactions:
                valid = validate_transaction(transaction[0], transaction[1])

                if valid:
                    #print(transaction)
                    transactions_total += transaction[0]


            print(f"Total transaction sum for {file_base}: {transactions_total}")

    print("")

# This can definitely be improved using pruning and evaluating as we recurse, if it gets bigger than the target, quit early
def pt2():
    print("Part 2:")

    for file_base in ["example", "input"]:
        with open(f"{file_base}.txt") as input_file:
            transactions = read_input(input_file)

            invalid = list()

            transactions_total: int = 0

            for transaction in transactions:
                valid = validate_transaction(transaction[0], transaction[1])

                if valid:
                    #print(transaction)
                    transactions_total += transaction[0]
                else:
                    invalid.append(transaction)

            for transaction in invalid:
                valid = validate_transaction(transaction[0], transaction[1], "", True)

                if valid:
                    #print(transaction)
                    transactions_total += transaction[0]



            print(f"Total transaction sum for {file_base}: {transactions_total}")

    print("")

def main():
    pt1()
    pt2()

if __name__ == "__main__":
    main()