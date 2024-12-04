import pathlib
import re

def solve_p2(path: str):
    contents = pathlib.Path(path).read_text()
    regex = re.compile(r"(mul\((\d+),(\d+)\)|(don't|do))")
    matches = [regex.findall(line) for line in contents.splitlines()]
    sum = 0
    enabled = True
    for line in matches:
        for match in line:
            if "mul" in match[0]:
                if enabled:
                    sum += int(match[1]) * int(match[2])
            elif "don't" in match[0]:
                enabled = False
            elif "do" in match[0]:
                enabled = True
    return sum

def solve_p1(path: str):
    contents = pathlib.Path(path).read_text()
    uncorrupted_muls = re.compile(r"mul\((\d+),(\d+)\)")
    muls = [uncorrupted_muls.findall(line) for line in contents.splitlines()]
    sum = 0
    for line in muls:
        for mul in line:
            left = int(mul[0])
            right = int(mul[1])
            sum += left * right
    return sum

def main():
    print(f"p1 example solution: {solve_p1('example.txt')}")
    print(f"p1 final solution: {solve_p1('input.txt')}")

    print(f"p2 example solution: {solve_p2('example2.txt')}")
    print(f"p2 final solution: {solve_p2('input.txt')}")

if __name__ == "__main__":
    main()
