import pathlib
import math
from copy import deepcopy

def sliding_window(iterable, window_size):
    for i in range(len(iterable) - window_size + 1):
        yield iterable[i:i + window_size]

def process_report_with_margin(levels: list[int]) -> bool:
    safe = False
    for i, _ in enumerate(levels):
        copy = deepcopy(levels)
        del copy[i]
        safe = process_report(copy) or safe
    return safe


def process_report(levels: list[int]) -> bool:
    increasing = all( a > b and (1 <= abs(a - b) <= 3) for [a, b] in sliding_window(levels, 2))
    decreasing = all( a < b and (1 <= abs(a - b) <= 3) for [a, b] in sliding_window(levels, 2))
    return increasing or decreasing

def preprocess_report(report: str) -> list[int]:
    return [int(level) for level in report.split(" ")]

def solve_p1(path: str):
    contents = pathlib.Path(path).read_text()
    reports = [preprocess_report(l) for l in contents.splitlines()]
    return [process_report(r) for r in reports].count(True)

def solve_p2(path: str):
    contents = pathlib.Path(path).read_text()
    reports = [preprocess_report(l) for l in contents.splitlines()]
    return [process_report_with_margin(r) for r in reports].count(True)


def main():
    print(f"p1 example solution: {solve_p1('example.txt')}")
    print(f"p1 final solution: {solve_p1('input.txt')}")
    print(f"p2 example solution: {solve_p2('example.txt')}")
    print(f"p2 final solution: {solve_p2('input.txt')}")

if __name__ == "__main__":
    main()
