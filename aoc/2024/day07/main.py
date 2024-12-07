import pathlib
import operator

from typing import TypeAlias, Callable
from dataclasses import dataclass, field
from itertools import product

@dataclass
class Equation():
    goal: int
    components: list[int]
    operators: list[Callable]

    def solve(self) -> bool:
        cartesian_product = product(
            self.operators,
            repeat=(len(self.components) - 1)
        )
        for op_permutation in cartesian_product:
            ops = list(op_permutation)
            running_total = self.components[0]
            for i, op in enumerate(ops):
                running_total = op(running_total, self.components[i+1])
            if running_total == self.goal:
                return True
        return False

def concat(a: int, b: int) -> int:
    return int(str(a) + str(b))

def parse_equations(block: str, operators: list[Callable]) -> list[Equation]:
    lines = block.splitlines()
    equations: list[Equation] = []
    for line in lines:
        [goal, components] = line.split(": ")
        nums = [int(num) for num in components.split(" ")]
        equations.append(Equation(
            int(goal),
            nums,
            operators = operators))

    return equations

def solve_p1(path: str):
    contents = pathlib.Path(path).read_text()
    eqs = parse_equations(contents, operators = [operator.add, operator.mul])
    return sum(eq.goal for eq in eqs if eq.solve())

def solve_p2(path: str):
    contents = pathlib.Path(path).read_text()
    eqs = parse_equations(contents, operators = [operator.add, operator.mul, concat])
    return sum(eq.goal for eq in eqs if eq.solve())

def main():
    print(f"p1 example solution: {solve_p1('example.txt')}")
    print(f"p1 final solution: {solve_p1('input.txt')}")
    print(f"p2 example solution: {solve_p2('example.txt')}")
    print(f"p2 final solution: {solve_p2('input.txt')}")

if __name__ == "__main__":
    main()
