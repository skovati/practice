import pathlib

from typing import TypeAlias
from dataclasses import dataclass
from itertools import permutations

Loc: TypeAlias = (int, int)
Grid: TypeAlias = list[list[str]]

def within_grid(grid: Grid, loc: Loc):
    return loc[0] in range(len(grid)) and loc[1] in range(len(grid[0]))


def get_antinodes(grid: Grid, loc1: Loc, loc2: Loc) -> list[Loc]:
    dy = loc2[0] - loc1[0]
    dx = loc2[1] - loc1[1]

    antinode1 = (loc1[0] - dy, loc1[1] - dx)
    antinode2 = (loc2[0] + dy, loc2[1] + dx)

    return [a for a in [antinode1, antinode2] if within_grid(grid, a)]


def solve_p1(path: str):
    contents = pathlib.Path(path).read_text()

    antinodes = set()
    nodes: dict[str, list[Loc]] = dict()

    grid = [list(line) for line in contents.splitlines()]
    for i, row in enumerate(grid):
        for j, col in enumerate(row):
            if col != ".":
                if col not in nodes:
                    nodes[col] = [(i, j)]
                else:
                    nodes.get(col, []).append((i, j))

    for id, nodes in nodes.items():
        print(f"for id: {id}, looking at permutations for nodes: {nodes}")
        for perm in permutations(nodes, 2):
            print(f"perm: {perm}")
            for a in get_antinodes(grid, perm[0], perm[1]):
                antinodes.add(a)

    return len(antinodes)

def main():
    print(f"p1 example solution: {solve_p1('example.txt')}")
    print(f"p1 final solution: {solve_p1('input.txt')}")
    # print(f"p2 example solution: {solve_p2('example.txt', parallelize=False)}")
    # print(f"p2 final solution: {solve_p2('input.txt')}")

if __name__ == "__main__":
    main()
