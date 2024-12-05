from pathlib import Path
from typing import List, Tuple

def read_matrix(path: str) -> List[List[str]]:
    return [list(line) for line in Path(path).read_text().splitlines()]

def is_valid(x: int, y: int, rows: int, cols: int) -> bool:
    return x in range(rows) and y in range(cols)

def check_mas(matrix: List[List[str]], x: int, y: int, dx: int, dy: int) -> bool:
    rows, cols = len(matrix), len(matrix[0])
    return (
        all(
            is_valid(x + i*dx, y + i*dy, rows, cols) and
            matrix[x + i*dx][y + i*dy] == "MAS"[i]
            for i in range(3)
        ) or all(
            is_valid(x + i*dx, y + i*dy, rows, cols) and
            matrix[x + i*dx][y + i*dy] == "SAM"[i]
            for i in range(3)
        )
    )

def solve_p2(path: str) -> int:
    matrix = read_matrix(path)
    rows, cols = len(matrix), len(matrix[0])

    def check_x_mas(x: int, y: int) -> bool:
        return (
            check_mas(matrix, x-1, y-1, 1, 1) and
            check_mas(matrix, x-1, y+1, 1, -1) and
            matrix[x][y] == 'A'
        )

    return sum(check_x_mas(x, y) for x in range(1, rows-1) for y in range(1, cols-1))

def solve_p1(path: str) -> int:
    matrix = read_matrix(path)
    rows, cols = len(matrix), len(matrix[0])

    directions = [
        (0, 1), (1, 0), (1, 1), (-1, 1),
        (0, -1), (-1, 0), (-1, -1), (1, -1)
    ]

    def check_xmas(row: int, col: int, dx: int, dy: int) -> bool:
        return all(
            is_valid(row + i*dx, col + i*dy, rows, cols) and
            matrix[row + i*dx][col + i*dy] == "XMAS"[i]
            for i in range(4)
        )

    return sum(
        check_xmas(i, j, dx, dy)
        for i in range(rows)
        for j in range(cols)
        for dx, dy in directions
    )

def main():
    for part, solve_func in [("p1", solve_p1), ("p2", solve_p2)]:
        for file in ["example.txt", "input.txt"]:
            print(f"{part} {file} solution: {solve_func(file)}")

if __name__ == "__main__":
    main()
